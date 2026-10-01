//! Everything that builds shell code for the Arcadia host goes through here.

use anyhow::{bail, Result};

/// Quotes `value` for a POSIX shell. Values made only of safe characters are left as is, so the
/// logged commands stay readable.
pub fn quote(value: &str) -> String {
    let safe = !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"-_./=:,@%+".contains(&byte));
    if safe {
        value.to_string()
    } else {
        format!("'{}'", value.replace('\'', r"'\''"))
    }
}

pub fn join<S: AsRef<str>>(args: &[S]) -> String {
    args.iter()
        .map(|arg| quote(arg.as_ref()))
        .collect::<Vec<_>>()
        .join(" ")
}

/// A value handed to a remote script through its stdin, exported as the variable `name`
#[derive(Clone, Copy)]
pub struct Secret<'a> {
    pub name: &'static str,
    pub value: &'a str,
}

/// Wraps `script` into a single `sh -c` argument for ssh. The script stops at the first failure,
/// and first reads the secrets from stdin, one line each and in order, into exported variables:
/// they never appear in a command line.
pub fn remote_command(script: &str, secrets: &[Secret]) -> String {
    let mut full = String::from("set -eu\n");
    for secret in secrets {
        full.push_str(&format!("IFS= read -r {0}\nexport {0}\n", secret.name));
    }
    full.push_str(script);
    format!("sh -c {}", quote(&full))
}

/// What to write on the stdin of a `remote_command`: the secrets, then `input`
pub fn stdin_payload(secrets: &[Secret], input: Option<&[u8]>) -> Result<Vec<u8>> {
    let mut payload = Vec::new();
    for secret in secrets {
        if secret.value.contains('\n') {
            bail!("the value of {} must not contain a line break", secret.name);
        }
        payload.extend_from_slice(secret.value.as_bytes());
        payload.push(b'\n');
    }
    if let Some(input) = input {
        payload.extend_from_slice(input);
    }
    Ok(payload)
}

/// Replaces `dst` with a copy of `src`: a directory is emptied then filled, so it ends up exactly
/// like `src`; a file is overwritten in place, keeping its inode for the bind mounts.
pub fn sync_script(src: &str, dst: &str) -> String {
    let (src, dst) = (quote(src), quote(dst));
    format!(
        "if [ -d {src} ]\nthen\nmkdir -p {dst}\nfind {dst} -mindepth 1 -delete\ncp -a {src}/. {dst}/\nelse\nmkdir -p \"$(dirname {dst})\"\ncp -a {src} {dst}\nfi"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::os::unix::fs::MetadataExt;
    use std::process::{Command, Stdio};

    /// Runs `command` the way sshd does: handed to a shell as a single string
    fn sh(command: &str, stdin: &[u8]) -> String {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(stdin).unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success(), "{command} failed");
        String::from_utf8(output.stdout).unwrap()
    }

    #[test]
    fn quote_leaves_safe_values_alone() {
        assert_eq!(quote("abc-1.2/x_y:z,@%+="), "abc-1.2/x_y:z,@%+=");
    }

    #[test]
    fn quote_wraps_everything_else() {
        assert_eq!(quote("a b"), "'a b'");
        assert_eq!(quote("it's"), r"'it'\''s'");
        assert_eq!(quote(""), "''");
    }

    #[test]
    fn quoted_values_survive_a_shell() {
        for value in [
            "a b",
            "it's",
            "$HOME",
            "`id`",
            "x\"y",
            "tab\tx",
            "new\nline",
            "*",
            "; rm -rf /",
        ] {
            assert_eq!(sh(&format!("printf %s {}", quote(value)), b""), value);
        }
    }

    #[test]
    fn join_quotes_each_argument() {
        assert_eq!(join(&["echo", "a b", "c"]), "echo 'a b' c");
    }

    #[test]
    fn secrets_reach_the_script_and_stay_out_of_the_command() {
        let secrets = [
            Secret {
                name: "FIRST",
                value: "p'a$s w\"d",
            },
            Secret {
                name: "SECOND",
                value: "",
            },
        ];
        let command = remote_command(r#"printf '%s|%s' "$FIRST" "$SECOND""#, &secrets);
        assert!(!command.contains("p'a$s"));
        let payload = stdin_payload(&secrets, None).unwrap();
        assert_eq!(sh(&command, &payload), "p'a$s w\"d|");
    }

    #[test]
    fn input_follows_the_secrets_on_stdin() {
        let secrets = [Secret {
            name: "TOKEN",
            value: "t",
        }];
        let command = remote_command(r#"printf '%s:' "$TOKEN"; cat"#, &secrets);
        let payload = stdin_payload(&secrets, Some(b"data")).unwrap();
        assert_eq!(sh(&command, &payload), "t:data");
    }

    #[test]
    fn remote_scripts_stop_at_the_first_failure() {
        let command = remote_command("false; echo reached", &[]);
        let status = Command::new("sh").arg("-c").arg(&command).status().unwrap();
        assert!(!status.success());
    }

    #[test]
    fn secrets_with_line_breaks_are_rejected() {
        let secrets = [Secret {
            name: "BROKEN",
            value: "a\nb",
        }];
        let error = stdin_payload(&secrets, None).unwrap_err().to_string();
        assert!(error.contains("BROKEN"), "{error}");
    }

    #[test]
    fn sync_script_replaces_a_directory_with_spaces_in_its_path() {
        let root = tempfile::tempdir().unwrap();
        let src = root.path().join("s rc");
        let dst = root.path().join("d'st");
        std::fs::create_dir_all(src.join("sub dir")).unwrap();
        std::fs::write(src.join("a"), "a").unwrap();
        std::fs::write(src.join("sub dir/b"), "b").unwrap();
        std::fs::create_dir_all(&dst).unwrap();
        std::fs::write(dst.join("stale"), "stale").unwrap();

        sh(
            &sync_script(src.to_str().unwrap(), dst.to_str().unwrap()),
            b"",
        );

        assert_eq!(std::fs::read_to_string(dst.join("a")).unwrap(), "a");
        assert_eq!(std::fs::read_to_string(dst.join("sub dir/b")).unwrap(), "b");
        assert!(!dst.join("stale").exists());
    }

    #[test]
    fn sync_script_overwrites_a_file_in_place() {
        let root = tempfile::tempdir().unwrap();
        let src = root.path().join("new");
        let dst = root.path().join("config dir/config.yml");
        std::fs::write(&src, "restored").unwrap();
        std::fs::create_dir_all(dst.parent().unwrap()).unwrap();
        std::fs::write(&dst, "current").unwrap();
        let inode = std::fs::metadata(&dst).unwrap().ino();

        sh(
            &sync_script(src.to_str().unwrap(), dst.to_str().unwrap()),
            b"",
        );

        assert_eq!(std::fs::read_to_string(&dst).unwrap(), "restored");
        assert_eq!(std::fs::metadata(&dst).unwrap().ino(), inode);
    }

    #[test]
    fn sync_script_creates_missing_parents() {
        let root = tempfile::tempdir().unwrap();
        let src = root.path().join("file");
        let dst = root.path().join("a/b/file");
        std::fs::write(&src, "x").unwrap();
        sh(
            &sync_script(src.to_str().unwrap(), dst.to_str().unwrap()),
            b"",
        );
        assert_eq!(std::fs::read_to_string(&dst).unwrap(), "x");
    }

    #[test]
    fn sync_script_stops_at_first_failure_directory_branch() {
        let root = tempfile::tempdir().unwrap();
        let src = root.path().join("dir");
        let blocking_file = root.path().join("file");
        let dst = root.path().join("file/dst");
        std::fs::create_dir(&src).unwrap();
        std::fs::write(&blocking_file, "blocking").unwrap();

        let command = remote_command(
            &format!(
                "{}\necho reached",
                sync_script(src.to_str().unwrap(), dst.to_str().unwrap())
            ),
            &[],
        );
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(b"").unwrap();
        let output = child.wait_with_output().unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();

        assert!(
            !output.status.success(),
            "sync script should fail when mkdir fails"
        );
        assert!(
            !stdout.contains("reached"),
            "script should not reach echo after sync_script fails"
        );
    }

    #[test]
    fn sync_script_stops_at_first_failure_file_branch() {
        let root = tempfile::tempdir().unwrap();
        let src = root.path().join("file");
        let blocking_file = root.path().join("file2");
        let dst = root.path().join("file2/dst");
        std::fs::write(&src, "source").unwrap();
        std::fs::write(&blocking_file, "blocking").unwrap();

        let command = remote_command(
            &format!(
                "{}\necho reached",
                sync_script(src.to_str().unwrap(), dst.to_str().unwrap())
            ),
            &[],
        );
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.take().unwrap().write_all(b"").unwrap();
        let output = child.wait_with_output().unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();

        assert!(
            !output.status.success(),
            "sync script should fail when mkdir fails"
        );
        assert!(
            !stdout.contains("reached"),
            "script should not reach echo after sync_script fails"
        );
    }
}
