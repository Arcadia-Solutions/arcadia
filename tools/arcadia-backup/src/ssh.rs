//! One multiplexed ssh connection to the Arcadia host, carrying the tunnel to the rest-server.

use crate::config::SshConfig;
use crate::process::KillOnDrop;
use crate::shell::{self, quote, Secret};
use anyhow::{anyhow, bail, Context, Result};
use std::ffi::OsString;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

pub struct Session {
    target: String,
    options: Vec<String>,
    socket: PathBuf,
    /// Port of the tunnel end on the Arcadia host (127.0.0.1)
    pub remote_port: u16,
    #[expect(dead_code, reason = "held for its Drop")]
    child: KillOnDrop,
    _dir: tempfile::TempDir,
}

/// Master ssh args for control socket (without the tunnel)
pub fn master_args(socket: &Path) -> Vec<OsString> {
    let mut args: Vec<OsString> = ["-N", "-M", "-S"].iter().map(OsString::from).collect();
    args.push(socket.into());
    for option in [
        "BatchMode=yes",
        "ConnectTimeout=30",
        "ServerAliveInterval=30",
    ] {
        args.push("-o".into());
        args.push(option.into());
    }
    args
}

/// `ssh -S <socket> <options> <op> <target>`: a command to the master, which applies the
/// same ssh options (`-F`, `-p`, ...) as the one that started it
pub fn control_args(socket: &Path, options: &[String], target: &str, op: &[&str]) -> Vec<OsString> {
    let mut args: Vec<OsString> = vec!["-S".into(), socket.into()];
    args.extend(options.iter().map(OsString::from));
    args.extend(op.iter().map(OsString::from));
    args.push(target.into());
    args
}

/// Args adding the reverse tunnel once the master is established. Port 0 on the remote end lets
/// sshd pick a free port it is allowed to bind (SELinux restricts which ones), which the mux
/// client prints on stdout.
pub fn forward_args(
    socket: &Path,
    options: &[String],
    target: &str,
    local_port: u16,
) -> Vec<OsString> {
    let spec = format!("127.0.0.1:0:127.0.0.1:{local_port}");
    control_args(socket, options, target, &["-O", "forward", "-R", &spec])
}

/// The remote port sshd allocated, from the stdout of `ssh -O forward -R ...:0:...`
pub fn parse_allocated_port(stdout: &str, stderr: &str) -> Result<u16> {
    match stdout.trim().parse::<u16>() {
        Ok(port) if port != 0 => Ok(port),
        _ => bail!(
            "ssh did not report the port allocated for the tunnel (stdout: {:?}):\n{}",
            stdout.trim(),
            stderr.trim()
        ),
    }
}

impl Session {
    pub fn open(config: &SshConfig, local_port: u16) -> Result<Session> {
        let dir = tempfile::Builder::new()
            .prefix("arcadia-backup-ssh-")
            .tempdir()?;
        let socket = dir.path().join("control");
        let log = dir.path().join("ssh.log");

        // Spawn the master process, wrapped in KillOnDrop for guaranteed cleanup
        let child = Command::new("ssh")
            .args(master_args(&socket))
            .args(&config.options)
            .arg(&config.target)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(std::fs::File::create(&log)?)
            .spawn()
            .context("cannot run ssh")?;
        let mut child = KillOnDrop::new(child);

        // Wait for the master to be ready by polling the control socket
        let deadline = Instant::now() + Duration::from_secs(35);
        loop {
            if let Some(_status) = child.try_wait()? {
                let error = std::fs::read_to_string(&log).unwrap_or_default();
                bail!(
                    "cannot connect to {} over ssh:\n{}",
                    config.target,
                    error.trim()
                );
            }
            if Instant::now() > deadline {
                bail!(
                    "cannot connect to {} over ssh (master did not become ready):\n{}",
                    config.target,
                    std::fs::read_to_string(&log).unwrap_or_default().trim()
                );
            }
            let status = Command::new("ssh")
                .args(control_args(
                    &socket,
                    &config.options,
                    &config.target,
                    &["-O", "check"],
                ))
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
            match status {
                Ok(s) if s.success() => break,
                Ok(_) => {}  // check failed, keep polling
                Err(_) => {} // command failed, keep polling
            }
            std::thread::sleep(Duration::from_millis(100));
        }

        // Now establish the tunnel; KillOnDrop kills the master on every error path
        let output = Command::new("ssh")
            .args(forward_args(
                &socket,
                &config.options,
                &config.target,
                local_port,
            ))
            .stdin(Stdio::null())
            .output()
            .context("cannot run ssh")?;
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !output.status.success() {
            bail!(
                "cannot establish port forwarding to {} over ssh:\n{}",
                config.target,
                stderr.trim()
            );
        }
        let remote_port = parse_allocated_port(&String::from_utf8_lossy(&output.stdout), &stderr)?;
        Ok(Session {
            target: config.target.clone(),
            options: config.options.clone(),
            socket,
            remote_port,
            child,
            _dir: dir,
        })
    }

    pub(crate) fn command(&self, remote: &str) -> Command {
        let mut command = Command::new("ssh");
        command
            .arg("-S")
            .arg(&self.socket)
            .arg("-o")
            .arg("BatchMode=yes")
            .args(&self.options)
            .arg(&self.target)
            .arg(remote);
        command
    }

    /// Runs `script` with `sh` on the Arcadia host, the secrets exported as environment variables
    /// and `input` on its stdin. Returns its stdout, its stderr goes to ours.
    pub fn run(&self, script: &str, secrets: &[Secret], input: Option<&[u8]>) -> Result<String> {
        let payload = shell::stdin_payload(secrets, input)?;
        let mut child = self
            .command(&shell::remote_command(script, secrets))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .context("cannot run ssh")?;
        let mut stdin = child.stdin.take().context("no stdin")?;
        // Written from a thread, the remote side may produce output before reading everything
        let writer = std::thread::spawn(move || {
            let _ = stdin.write_all(&payload);
        });
        let output = child.wait_with_output()?;
        writer
            .join()
            .map_err(|_| anyhow!("the stdin writer panicked"))?;
        if !output.status.success() {
            let summary: String = script
                .lines()
                .next()
                .unwrap_or_default()
                .chars()
                .take(200)
                .collect();
            bail!(
                "command failed on the Arcadia host ({}): {summary}",
                output.status
            );
        }
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let _ = Command::new("ssh")
            .args(control_args(
                &self.socket,
                &self.options,
                &self.target,
                &["-O", "exit"],
            ))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        // child will be killed and waited by KillOnDrop drop when this scope ends
    }
}

fn lock_script(work_dir: &str) -> String {
    let dir = quote(work_dir);
    format!(
        "command -v flock >/dev/null 2>&1 || {{ echo noflock; exit 0; }}\nmkdir -p {dir} && chmod 700 {dir}\nexec 9>{dir}/lock\nif ! flock -n 9; then echo busy; exit 0; fi\necho locked\nexec cat >/dev/null"
    )
}

/// `flock` held by a remote process living until we close its stdin, or until the connection
/// drops: a crashed run never leaves a stale lock behind.
pub struct RemoteLock {
    #[expect(dead_code, reason = "held for its Drop")]
    child: KillOnDrop,
    stdin: Option<std::process::ChildStdin>,
}

impl RemoteLock {
    pub fn acquire(session: &Session, work_dir: &str) -> Result<RemoteLock> {
        let mut child = session
            .command(&shell::remote_command(&lock_script(work_dir), &[]))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .context("cannot run ssh")?;
        let stdin = child.stdin.take();
        let mut line = String::new();
        BufReader::new(child.stdout.take().context("no stdout")?).read_line(&mut line)?;
        match line.trim() {
            "locked" => Ok(RemoteLock {
                child: KillOnDrop::new(child),
                stdin,
            }),
            "busy" => {
                let _ = child.wait();
                bail!("another arcadia-backup run holds {work_dir}/lock on the Arcadia host")
            }
            "noflock" => {
                let _ = child.wait();
                bail!("flock is not installed on the Arcadia host (util-linux)")
            }
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                bail!("cannot take the lock {work_dir}/lock on the Arcadia host (unexpected response)")
            }
        }
    }
}

impl Drop for RemoteLock {
    fn drop(&mut self) {
        drop(self.stdin.take());
        // child will be killed and waited by KillOnDrop drop when this scope ends
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn master_args_are_control_socket_without_forward() {
        let args: Vec<String> = master_args(Path::new("/tmp/s/control"))
            .into_iter()
            .map(|a| a.into_string().unwrap())
            .collect();
        assert_eq!(
            args,
            vec![
                "-N",
                "-M",
                "-S",
                "/tmp/s/control",
                "-o",
                "BatchMode=yes",
                "-o",
                "ConnectTimeout=30",
                "-o",
                "ServerAliveInterval=30",
            ]
        );
    }

    #[test]
    fn forward_args_establish_the_tunnel() {
        let options = ["-F".to_string(), "/x/config".to_string()];
        let args: Vec<String> =
            forward_args(Path::new("/tmp/s/control"), &options, "me@host", 41000)
                .into_iter()
                .map(|a| a.into_string().unwrap())
                .collect();
        assert_eq!(
            args,
            vec![
                "-S",
                "/tmp/s/control",
                "-F",
                "/x/config",
                "-O",
                "forward",
                "-R",
                "127.0.0.1:0:127.0.0.1:41000",
                "me@host",
            ]
        );
    }

    #[test]
    fn allocated_port_is_read_from_stdout() {
        assert_eq!(parse_allocated_port("44785\n", "").unwrap(), 44785);
    }

    #[test]
    fn missing_or_garbage_allocated_port_is_an_error_with_stderr() {
        for stdout in ["", "\n", "0\n", "abc\n", "70000\n"] {
            let error = parse_allocated_port(stdout, "boom\n")
                .unwrap_err()
                .to_string();
            assert!(error.contains("boom"), "{error}");
        }
    }

    #[test]
    fn control_commands_carry_the_ssh_options_before_the_target() {
        let options = ["-p".to_string(), "2222".to_string()];
        let args: Vec<String> = control_args(Path::new("/s"), &options, "me@host", &["-O", "exit"])
            .into_iter()
            .map(|a| a.into_string().unwrap())
            .collect();
        assert_eq!(
            args,
            vec!["-S", "/s", "-p", "2222", "-O", "exit", "me@host"]
        );
    }

    #[test]
    fn lock_script_holds_flock_until_stdin_closes() {
        let script = lock_script("/opt/arcadia/.arcadia-backup");
        assert!(script.contains("command -v flock >/dev/null 2>&1"));
        assert!(script.contains("exec 9>/opt/arcadia/.arcadia-backup/lock"));
        assert!(script.contains("flock -n 9"));
        assert!(script.ends_with("exec cat >/dev/null"));
    }
}
