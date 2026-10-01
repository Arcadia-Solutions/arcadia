//! restic invocations: local ones on the backup host, and the parsing of their output.

use crate::components::Source;
use crate::config::{Config, Retention, Runner};
use anyhow::{bail, Context, Result};
use chrono::{DateTime, FixedOffset};
use serde::Deserialize;
use std::path::Path;
use std::process::{Command, Stdio};

/// Tag of every snapshot made by arcadia-backup
pub const TAG: &str = "arcadia";

/// Where meta.json is in the snapshots made with the docker runner
pub const DOCKER_META_PATH: &str = "/arcadia/meta/meta.json";

/// Named volume keeping the restic cache across runs, so the index is not downloaded every time
pub const CACHE_VOLUME: &str = "arcadia-backup-restic-cache";

/// Handed on stdin by `Session::run`, read from the environment by restic
pub const SECRET_NAMES: [&str; 3] = [
    "RESTIC_PASSWORD",
    "RESTIC_REST_USERNAME",
    "RESTIC_REST_PASSWORD",
];

#[derive(Debug, Clone, Deserialize)]
pub struct Snapshot {
    pub id: String,
    pub short_id: String,
    pub time: DateTime<FixedOffset>,
    #[expect(
        dead_code,
        reason = "part of the restic JSON, filtered on by the restic call"
    )]
    pub hostname: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

pub fn sort_latest_first(snapshots: &[Snapshot]) -> Vec<&Snapshot> {
    let mut sorted: Vec<&Snapshot> = snapshots.iter().collect();
    sorted.sort_by(|a, b| b.time.cmp(&a.time));
    sorted
}

/// Snapshots dated further in the future than this are not trusted
const MAX_CLOCK_SKEW_SECONDS: i64 = 3600;

fn same_snapshot(id: &str, other: &str) -> bool {
    !id.is_empty() && !other.is_empty() && (id.starts_with(other) || other.starts_with(id))
}

/// Why the retention policy must not be applied now. The Arcadia host could plant snapshots
/// dated in the future, which `forget` would count as the newest and keep instead of the real
/// history. A backup adds exactly one snapshot, the one named in its summary.
pub fn retention_blockers(
    before: &[Snapshot],
    after: &[Snapshot],
    created_id: &str,
    now: DateTime<FixedOffset>,
) -> Vec<String> {
    let mut blockers = Vec::new();
    let unexpected: Vec<&str> = after
        .iter()
        .filter(|snapshot| !before.iter().any(|old| old.id == snapshot.id))
        .filter(|snapshot| !same_snapshot(&snapshot.id, created_id))
        .map(|snapshot| snapshot.short_id.as_str())
        .collect();
    if !unexpected.is_empty() {
        blockers.push(format!(
            "more than one snapshot appeared during the backup, besides the one it made: {}",
            unexpected.join(", ")
        ));
    }
    blockers.extend(future_snapshots(after, now));
    blockers
}

/// One message per snapshot dated more than an hour in the future
pub fn future_snapshots(snapshots: &[Snapshot], now: DateTime<FixedOffset>) -> Vec<String> {
    snapshots
        .iter()
        .filter(|snapshot| (snapshot.time - now).num_seconds() > MAX_CLOCK_SKEW_SECONDS)
        .map(|snapshot| {
            format!(
                "snapshot {} is dated in the future ({})",
                snapshot.short_id,
                snapshot.time.format("%Y-%m-%d %H:%M:%S %z")
            )
        })
        .collect()
}

pub fn forget_args(host: &str, retention: &Retention) -> Vec<String> {
    let mut args: Vec<String> = [
        "forget",
        "--prune",
        "--host",
        host,
        "--tag",
        TAG,
        "--group-by",
        "host",
    ]
    .iter()
    .map(|arg| arg.to_string())
    .collect();
    args.extend(retention.forget_args());
    args
}

/// restic run on the backup host, directly against the repository
pub struct LocalRestic<'a> {
    config: &'a Config,
}

impl<'a> LocalRestic<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    fn output<S: AsRef<str>>(&self, args: &[S]) -> Result<Vec<u8>> {
        self.output_with(args, Stdio::inherit())
    }

    fn output_with<S: AsRef<str>>(&self, args: &[S], stderr: Stdio) -> Result<Vec<u8>> {
        let restic = &self.config.restic;
        let output = Command::new(&restic.binary)
            .arg("--repo")
            .arg(&restic.repository)
            .env("RESTIC_PASSWORD_FILE", &restic.password_file)
            .args(args.iter().map(AsRef::as_ref))
            .stdin(Stdio::null())
            .stderr(stderr)
            .output()
            .with_context(|| format!("cannot run '{}', is restic installed?", restic.binary))?;
        if !output.status.success() {
            let command = args.first().map(AsRef::as_ref).unwrap_or_default();
            bail!("restic {command} failed ({})", output.status);
        }
        Ok(output.stdout)
    }

    pub fn init(&self) -> Result<()> {
        self.output(&["init"]).map(|_| ())
    }

    pub fn snapshots(&self, host: &str) -> Result<Vec<Snapshot>> {
        let output = self.output(&["snapshots", "--json", "--host", host, "--tag", TAG])?;
        serde_json::from_slice(&output).context("cannot parse the output of restic snapshots")
    }

    pub fn forget(&self, host: &str, retention: &Retention) -> Result<()> {
        self.output(&forget_args(host, retention)).map(|_| ())
    }

    pub fn check(&self, read_data_subset: Option<&str>) -> Result<()> {
        let mut args = vec!["check".to_string()];
        if let Some(subset) = read_data_subset {
            args.push(format!("--read-data-subset={subset}"));
        }
        self.output(&args).map(|_| ())
    }

    pub fn dump(&self, id: &str, path: &str) -> Result<Vec<u8>> {
        self.output(&["dump", id, path])
    }

    /// Restores the whole snapshot into `target`, on this host
    pub fn restore_to(&self, id: &str, target: &Path) -> Result<()> {
        let target = target.to_str().context("the target is not valid utf-8")?;
        self.output(&["restore", id, "--target", target])
            .map(|_| ())
    }

    /// Like `dump`, for a path that may not exist: restic's complaint is not shown
    fn try_dump(&self, id: &str, path: &str) -> Option<Vec<u8>> {
        self.output_with(&["dump", id, path], Stdio::null()).ok()
    }

    /// meta.json of a snapshot: the two places it is written to are tried first, a listing of
    /// the whole snapshot is the last resort
    pub fn meta_json(&self, id: &str, work_dir: &str) -> Result<Vec<u8>> {
        let staged = format!("{work_dir}/staging/meta.json");
        if let Some(meta) = self
            .try_dump(id, DOCKER_META_PATH)
            .or_else(|| self.try_dump(id, &staged))
        {
            return Ok(meta);
        }
        let path = find_meta_path(&self.ls(id)?)
            .context("the snapshot has no meta.json, it was not made by arcadia-backup")?;
        self.dump(id, &path)
    }

    pub fn ls(&self, id: &str) -> Result<String> {
        String::from_utf8(self.output(&["ls", "--json", id])?)
            .context("restic ls printed invalid utf-8")
    }
}

/// restic run on the Arcadia host, reaching the repository through the ssh tunnel
pub struct RemoteRestic<'a> {
    pub config: &'a Config,
    /// Tunnel end on the Arcadia host
    pub port: u16,
}

impl RemoteRestic<'_> {
    fn repository(&self) -> String {
        format!("rest:http://127.0.0.1:{}/", self.port)
    }

    /// `docker run` of the restic image, with the host network to reach the tunnel end
    fn docker_prefix(&self, mounts: &[String]) -> Vec<String> {
        let mut args: Vec<String> = ["docker", "run", "--rm", "--network", "host"]
            .iter()
            .map(|a| a.to_string())
            .collect();
        for name in SECRET_NAMES {
            // without a value, docker takes the one of its own environment
            args.extend(["-e".to_string(), name.to_string()]);
        }
        args.extend([
            "-v".to_string(),
            format!("{CACHE_VOLUME}:/root/.cache/restic"),
        ]);
        for mount in mounts {
            args.extend(["-v".to_string(), mount.clone()]);
        }
        args.push(self.config.restic.image.clone());
        args
    }

    fn command(&self, mounts: &[String], restic_args: Vec<String>) -> String {
        let mut args = match self.config.restic.runner {
            Runner::Docker => self.docker_prefix(mounts),
            Runner::Binary => vec![self.config.restic.remote_binary.clone()],
        };
        args.extend(["--repo".to_string(), self.repository()]);
        args.extend(restic_args);
        crate::shell::join(&args)
    }

    /// One `restic backup` of every entry, `(source, snapshot path)`
    pub fn backup_script(
        &self,
        entries: &[(Source, String)],
        host: &str,
        tags: &[String],
    ) -> Result<String> {
        let mut restic_args: Vec<String> = ["backup", "--json", "--host", host]
            .iter()
            .map(|a| a.to_string())
            .collect();
        for tag in tags {
            restic_args.extend(["--tag".to_string(), tag.clone()]);
        }
        let mut mounts = Vec::new();
        for (source, snapshot_path) in entries {
            match self.config.restic.runner {
                Runner::Docker => {
                    check_mountable(source.mount_source())?;
                    check_mountable(snapshot_path)?;
                    mounts.push(format!("{}:{snapshot_path}:ro", source.mount_source()));
                }
                Runner::Binary => match source {
                    Source::Path(path) if path == snapshot_path => {}
                    _ => bail!(
                        "the binary runner can only back up host paths, not '{}'",
                        source.mount_source()
                    ),
                },
            }
            restic_args.push(snapshot_path.clone());
        }
        Ok(self.command(&mounts, restic_args))
    }

    /// Restores the whole snapshot in `root` on the Arcadia host
    pub fn restore_script(&self, id: &str, root: &str) -> Result<String> {
        let (mounts, target) = match self.config.restic.runner {
            Runner::Docker => {
                check_mountable(root)?;
                (vec![format!("{root}:/restore")], "/restore".to_string())
            }
            Runner::Binary => (vec![], root.to_string()),
        };
        Ok(self.command(
            &mounts,
            vec!["restore".into(), id.into(), "--target".into(), target],
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct BackupSummary {
    pub snapshot_id: String,
    pub data_added: u64,
}

/// `restic backup --json` prints status lines, then a summary line
pub fn parse_backup_summary(output: &str) -> Result<BackupSummary> {
    output
        .lines()
        .rev()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .find(|value| value.get("message_type").and_then(|t| t.as_str()) == Some("summary"))
        .map(serde_json::from_value)
        .transpose()?
        .context("restic backup did not print a summary")
}

#[derive(Deserialize)]
struct LsLine {
    #[serde(default)]
    struct_type: String,
    #[serde(default)]
    message_type: String,
    #[serde(default, rename = "type")]
    node_type: String,
    #[serde(default)]
    path: String,
}

/// Locates meta.json in the output of `restic ls --json`
pub fn find_meta_path(ls_output: &str) -> Option<String> {
    ls_output
        .lines()
        .filter_map(|line| serde_json::from_str::<LsLine>(line).ok())
        .filter(|line| line.struct_type == "node" || line.message_type == "node")
        .find(|line| {
            line.node_type == "file"
                && (line.path == DOCKER_META_PATH || line.path.ends_with("/staging/meta.json"))
        })
        .map(|line| line.path)
}

/// docker `-v` splits on ':', refuse such paths instead of mounting something else
pub fn check_mountable(path: &str) -> Result<()> {
    if path.contains(':') {
        bail!("'{path}' contains ':', docker cannot mount it");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::Source;
    use crate::config::test_config;
    use crate::config::Retention;

    #[test]
    fn parses_restic_snapshots_json() {
        let json = r#"[{"time":"2026-09-30T03:00:01.123456789+02:00","tree":"t","paths":["/arcadia"],
            "hostname":"prod","username":"root","tags":["arcadia","mode:docker"],
            "id":"4bba301e2b7d5b7a8b1a4a5d3e1b1a2c","short_id":"4bba301e"}]"#;
        let snapshots: Vec<Snapshot> = serde_json::from_str(json).unwrap();
        assert_eq!(snapshots[0].short_id, "4bba301e");
        assert_eq!(snapshots[0].tags, vec!["arcadia", "mode:docker"]);
    }

    fn snapshot(id: &str, time: &str) -> Snapshot {
        Snapshot {
            id: id.to_string(),
            short_id: id.chars().take(8).collect(),
            time: DateTime::parse_from_rfc3339(time).unwrap(),
            hostname: "prod".into(),
            tags: vec![],
        }
    }

    fn now() -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339("2026-09-30T12:00:00+00:00").unwrap()
    }

    #[test]
    fn retention_accepts_exactly_the_snapshot_the_backup_made() {
        let old = snapshot("aaaaaaaa11", "2026-09-29T12:00:00+00:00");
        let new = snapshot("bbbbbbbb22", "2026-09-30T11:59:00+00:00");
        assert!(retention_blockers(
            std::slice::from_ref(&old),
            &[old.clone(), new.clone()],
            "bbbbbbbb22",
            now()
        )
        .is_empty());
        // the summary may carry the short id
        assert!(retention_blockers(
            std::slice::from_ref(&old),
            &[old.clone(), new],
            "bbbbbbbb",
            now()
        )
        .is_empty());
    }

    #[test]
    fn retention_is_blocked_by_an_extra_new_snapshot() {
        let old = snapshot("aaaaaaaa11", "2026-09-29T12:00:00+00:00");
        let new = snapshot("bbbbbbbb22", "2026-09-30T11:59:00+00:00");
        let planted = snapshot("cccccccc33", "2026-09-30T11:59:30+00:00");
        let blockers = retention_blockers(
            std::slice::from_ref(&old),
            &[old.clone(), new, planted],
            "bbbbbbbb22",
            now(),
        );
        assert_eq!(blockers.len(), 1, "{blockers:?}");
        assert!(blockers[0].contains("cccccccc"), "{blockers:?}");
    }

    #[test]
    fn retention_is_blocked_by_snapshots_from_the_future() {
        let old = snapshot("aaaaaaaa11", "2026-09-29T12:00:00+00:00");
        let new = snapshot("bbbbbbbb22", "2026-09-30T11:59:00+00:00");
        let future = snapshot("dddddddd44", "2026-10-30T12:00:00+00:00");
        let blockers = retention_blockers(
            &[old.clone(), future.clone()],
            &[old, future.clone(), new],
            "bbbbbbbb22",
            now(),
        );
        assert_eq!(blockers.len(), 1, "{blockers:?}");
        assert!(blockers[0].contains("dddddddd") && blockers[0].contains("future"));
        // within the allowed skew
        let skewed = snapshot("eeeeeeee55", "2026-09-30T12:30:00+00:00");
        assert!(future_snapshots(&[skewed], now()).is_empty());
    }

    #[test]
    fn forget_args_scope_the_policy_to_the_host() {
        let retention = Retention {
            keep_daily: Some(7),
            ..Default::default()
        };
        assert_eq!(
            forget_args("prod", &retention),
            vec![
                "forget",
                "--prune",
                "--host",
                "prod",
                "--tag",
                "arcadia",
                "--group-by",
                "host",
                "--keep-daily",
                "7"
            ]
        );
    }

    #[test]
    fn parses_the_backup_summary() {
        let output = concat!(
            r#"{"message_type":"status","percent_done":0.5}"#,
            "\n",
            r#"{"message_type":"summary","files_new":3,"data_added":1234,"snapshot_id":"4bba301e2b7d"}"#,
            "\n",
        );
        let summary = parse_backup_summary(output).unwrap();
        assert_eq!(summary.snapshot_id, "4bba301e2b7d");
        assert_eq!(summary.data_added, 1234);
        assert!(parse_backup_summary(r#"{"message_type":"status"}"#).is_err());
    }

    #[test]
    fn finds_meta_json_in_both_layouts() {
        let docker = concat!(
            r#"{"time":"x","struct_type":"snapshot","message_type":"snapshot","id":"a"}"#,
            "\n",
            r#"{"name":"config.yml","type":"file","path":"/arcadia/config/config.yml","struct_type":"node","message_type":"node"}"#,
            "\n",
            r#"{"name":"meta.json","type":"file","path":"/arcadia/meta/meta.json","struct_type":"node","message_type":"node"}"#,
            "\n",
        );
        assert_eq!(find_meta_path(docker).as_deref(), Some(DOCKER_META_PATH));
        let binary = r#"{"name":"meta.json","type":"file","path":"/srv/arcadia/.arcadia-backup/staging/meta.json","message_type":"node"}"#;
        assert_eq!(
            find_meta_path(binary).as_deref(),
            Some("/srv/arcadia/.arcadia-backup/staging/meta.json")
        );
        assert_eq!(
            find_meta_path(
                r#"{"name":"meta.json","type":"file","path":"/elsewhere/meta.json","struct_type":"node"}"#
            ),
            None
        );
    }

    fn entries() -> Vec<(Source, String)> {
        vec![
            (
                Source::Path("/opt/arcadia/config.yml".into()),
                "/arcadia/config/config.yml".into(),
            ),
            (
                Source::Volume("arcadia_chevereto_storage".into()),
                "/arcadia/chevereto/files".into(),
            ),
        ]
    }

    #[test]
    fn docker_backup_mounts_everything_read_only() {
        let config = test_config("docker", "docker");
        let remote = RemoteRestic {
            config: &config,
            port: 40000,
        };
        let tags = vec!["arcadia".to_string(), "mode:docker".to_string()];
        assert_eq!(
            remote.backup_script(&entries(), "prod", &tags).unwrap(),
            "docker run --rm --network host -e RESTIC_PASSWORD -e RESTIC_REST_USERNAME -e RESTIC_REST_PASSWORD \
             -v arcadia-backup-restic-cache:/root/.cache/restic \
             -v /opt/arcadia/config.yml:/arcadia/config/config.yml:ro \
             -v arcadia_chevereto_storage:/arcadia/chevereto/files:ro \
             restic/restic:0.19.1 --repo rest:http://127.0.0.1:40000/ backup --json --host prod \
             --tag arcadia --tag mode:docker /arcadia/config/config.yml /arcadia/chevereto/files"
        );
    }

    #[test]
    fn binary_backup_uses_host_paths() {
        let config = test_config("standard", "binary");
        let remote = RemoteRestic {
            config: &config,
            port: 40000,
        };
        let entries = vec![(
            Source::Path("/w/staging/postgres.sql".into()),
            "/w/staging/postgres.sql".into(),
        )];
        assert_eq!(
            remote
                .backup_script(&entries, "prod", &["arcadia".to_string()])
                .unwrap(),
            "restic --repo rest:http://127.0.0.1:40000/ backup --json --host prod --tag arcadia /w/staging/postgres.sql"
        );
        let volume = vec![(Source::Volume("v".into()), "/v".into())];
        assert!(remote.backup_script(&volume, "prod", &[]).is_err());
    }

    #[test]
    fn docker_backup_refuses_paths_docker_cannot_mount() {
        let config = test_config("docker", "docker");
        let remote = RemoteRestic {
            config: &config,
            port: 40000,
        };
        let entries = vec![(
            Source::Path("/opt/a:b".into()),
            "/arcadia/extra/opt/a:b".into(),
        )];
        assert!(remote.backup_script(&entries, "prod", &[]).is_err());
    }

    #[test]
    fn restore_scripts_target_the_restore_root() {
        let config = test_config("docker", "docker");
        let script = RemoteRestic {
            config: &config,
            port: 40000,
        }
        .restore_script("4bba301e", "/w/restore")
        .unwrap();
        assert!(
            script.contains(
                "-v /w/restore:/restore restic/restic:0.19.1 --repo rest:http://127.0.0.1:40000/ restore 4bba301e --target /restore"
            ),
            "{script}"
        );
        let config = test_config("standard", "binary");
        let script = RemoteRestic {
            config: &config,
            port: 40000,
        }
        .restore_script("4bba301e", "/w/restore")
        .unwrap();
        assert_eq!(
            script,
            "restic --repo rest:http://127.0.0.1:40000/ restore 4bba301e --target /w/restore"
        );
    }

    #[test]
    fn no_secret_is_ever_in_the_scripts() {
        let config = test_config("docker", "docker");
        let script = RemoteRestic {
            config: &config,
            port: 40000,
        }
        .backup_script(&entries(), "prod", &[])
        .unwrap();
        for name in SECRET_NAMES {
            assert!(!script.contains(&format!("{name}=")), "{script}");
        }
    }
}
