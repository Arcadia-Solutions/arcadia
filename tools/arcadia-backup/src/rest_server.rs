//! The rest-server serving the repository to the Arcadia host, through the ssh tunnel.

use crate::process::KillOnDrop;
use anyhow::{bail, Context, Result};
use std::ffi::OsString;
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
use std::time::{Duration, Instant};

pub const USERNAME: &str = "arcadia-backup";

pub struct RestServer {
    // Drop order: child kills when it's dropped, then tempdir
    #[expect(dead_code, reason = "held for its Drop")]
    child: KillOnDrop,
    pub port: u16,
    /// Random, valid for this run only
    pub password: String,
    _dir: tempfile::TempDir,
}

/// Append only: the Arcadia host can add snapshots, never delete or rewrite them
pub fn args(repository: &Path, port: u16, htpasswd: &Path) -> Vec<OsString> {
    vec![
        "--path".into(),
        repository.into(),
        "--listen".into(),
        format!("127.0.0.1:{port}").into(),
        "--append-only".into(),
        "--htpasswd-file".into(),
        htpasswd.into(),
    ]
}

fn htpasswd_line(password: &str) -> Result<String> {
    let hash = bcrypt::hash_with_result(password, 10)?.format_for_version(bcrypt::Version::TwoY);
    Ok(format!("{USERNAME}:{hash}\n"))
}

impl RestServer {
    pub fn start(binary: &str, repository: &Path) -> Result<RestServer> {
        if !repository.join("config").is_file() {
            bail!(
                "'{}' is not a restic repository, run `arcadia-backup init` first",
                repository.display()
            );
        }
        let port = TcpListener::bind("127.0.0.1:0")?.local_addr()?.port();
        let password = crate::random::hex(24)?;
        let dir = tempfile::Builder::new()
            .prefix("arcadia-backup-")
            .tempdir()?;
        let htpasswd = dir.path().join("htpasswd");
        std::fs::write(&htpasswd, htpasswd_line(&password)?)?;

        let child = Command::new(binary)
            .args(args(repository, port, &htpasswd))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .spawn()
            .with_context(|| format!("cannot run '{binary}', is rest-server installed?"))?;
        let mut child = KillOnDrop::new(child);

        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            if TcpStream::connect(("127.0.0.1", port)).is_ok() {
                // Verify the process hasn't exited (another process may have taken the port)
                if let Some(status) = child.try_wait()? {
                    bail!("rest-server exited ({status})");
                }
                return Ok(RestServer {
                    child,
                    port,
                    password,
                    _dir: dir,
                });
            }
            if let Some(status) = child.try_wait()? {
                bail!("rest-server exited ({status})");
            }
            if Instant::now() > deadline {
                bail!("rest-server did not listen on 127.0.0.1:{port} within 10 seconds");
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

impl Drop for RestServer {
    fn drop(&mut self) {
        // child will be killed and waited by KillOnDrop drop when this scope ends
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serves_the_repository_append_only_on_localhost() {
        let args: Vec<String> = args(Path::new("/srv/repo"), 41000, Path::new("/tmp/x/htpasswd"))
            .into_iter()
            .map(|a| a.into_string().unwrap())
            .collect();
        assert_eq!(
            args,
            vec![
                "--path",
                "/srv/repo",
                "--listen",
                "127.0.0.1:41000",
                "--append-only",
                "--htpasswd-file",
                "/tmp/x/htpasswd"
            ]
        );
    }

    #[test]
    fn htpasswd_line_uses_bcrypt_2y() {
        let line = htpasswd_line("secret").unwrap();
        let hash = line.strip_prefix("arcadia-backup:").unwrap().trim_end();
        assert!(hash.starts_with("$2y$"), "{hash}");
        assert!(bcrypt::verify("secret", hash).unwrap());
    }
}
