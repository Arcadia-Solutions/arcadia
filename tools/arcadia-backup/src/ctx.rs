//! What the components need to reach the Arcadia host.

use crate::arcadia_config::{self, ArcadiaFile};
use crate::config::{Config, Mode, Runner};
use crate::restic::check_mountable;
use crate::shell::{self, quote, Secret};
use crate::ssh::Session;
use anyhow::{bail, Context, Result};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

pub fn compose(args: &[&str]) -> String {
    format!("docker compose {}", shell::join(args))
}

pub fn in_dir(dir: &str, script: &str) -> String {
    format!("cd {} && {script}", quote(dir))
}

pub fn parse_running(output: &str) -> BTreeSet<String> {
    output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect()
}

/// Compose volume key → docker volume name, from `docker compose config --format json`
pub fn parse_volumes(json: &str) -> Result<BTreeMap<String, String>> {
    #[derive(Deserialize)]
    struct ComposeConfig {
        #[serde(default)]
        volumes: BTreeMap<String, Volume>,
    }
    #[derive(Deserialize)]
    struct Volume {
        name: Option<String>,
    }
    let config: ComposeConfig =
        serde_json::from_str(json).context("cannot parse docker compose config")?;
    Ok(config
        .volumes
        .into_iter()
        .map(|(key, volume)| {
            let name = volume.name.unwrap_or_else(|| key.clone());
            (key, name)
        })
        .collect())
}

#[derive(Debug, PartialEq, Eq)]
pub enum ServiceState {
    Running,
    /// A container exists, in any other state (exited, created, restarting, ...)
    Stopped,
    /// Never created
    Absent,
}

#[derive(Debug, Default)]
pub struct Compose {
    pub running: BTreeSet<String>,
    /// Services with a container, running or not
    pub existing: BTreeSet<String>,
    pub volumes: BTreeMap<String, String>,
}

impl Compose {
    pub fn state(&self, service: &str) -> ServiceState {
        if self.running.contains(service) {
            ServiceState::Running
        } else if self.existing.contains(service) {
            ServiceState::Stopped
        } else {
            ServiceState::Absent
        }
    }

    pub fn load(config: &Config, session: &Session) -> Result<Compose> {
        let running = session.run(
            &in_dir(
                config.dir(),
                &compose(&["ps", "--services", "--status", "running"]),
            ),
            &[],
            None,
        )?;
        let existing = session.run(
            &in_dir(
                config.dir(),
                &compose(&["--profile", "*", "ps", "-a", "--services"]),
            ),
            &[],
            None,
        )?;
        // every profile, so that the volumes of stopped optional services are known too
        let volumes = session.run(
            &in_dir(
                config.dir(),
                &compose(&["--profile", "*", "config", "--format", "json"]),
            ),
            &[],
            None,
        )?;
        Ok(Compose {
            running: parse_running(&running),
            existing: parse_running(&existing),
            volumes: parse_volumes(&volumes)?,
        })
    }
}

/// Where a restored item goes
#[derive(Debug)]
pub enum Dest {
    Path(String),
    Volume(String),
}

fn split_parent(path: &str) -> Result<(&str, &str)> {
    match path.rsplit_once('/') {
        Some((parent, name)) if !name.is_empty() => {
            Ok((if parent.is_empty() { "/" } else { parent }, name))
        }
        _ => bail!("cannot restore to '{path}'"),
    }
}

/// Command replacing `dest` with the item at `snapshot_path` of the snapshot restored in `root`.
/// With the docker runner it runs in a root container, which keeps the ownership of the snapshot.
pub fn sync_command(
    config: &Config,
    root: &str,
    snapshot_path: &str,
    dest: &Dest,
) -> Result<String> {
    match config.restic.runner {
        Runner::Binary => match dest {
            Dest::Path(path) => Ok(shell::sync_script(&format!("{root}{snapshot_path}"), path)),
            Dest::Volume(volume) => {
                bail!("the volume '{volume}' can only be restored with restic.runner: docker")
            }
        },
        Runner::Docker => {
            check_mountable(root)?;
            let (setup, mount, target) = match dest {
                Dest::Volume(volume) => (
                    String::new(),
                    format!("{volume}:/target"),
                    "/target".to_string(),
                ),
                Dest::Path(path) => {
                    check_mountable(path)?;
                    let (parent, name) = split_parent(path)?;
                    (
                        format!("mkdir -p {} && ", quote(parent)),
                        format!("{parent}:/target"),
                        format!("/target/{name}"),
                    )
                }
            };
            let script = shell::sync_script(&format!("/restore{snapshot_path}"), &target);
            let restore_mount = format!("{root}:/restore:ro");
            Ok(format!(
                "{setup}{}",
                root_container(config, &[restore_mount, mount], &script)?
            ))
        }
    }
}

/// The script run by the container's `sh`: stops at the first failing command
pub fn container_script(script: &str) -> String {
    format!("set -eu\n{script}")
}

/// `docker run` of `script` in a root container of the restic image, mounting `mounts`
/// (`<host side>:<container side>[:ro]`)
pub fn root_container(config: &Config, mounts: &[String], script: &str) -> Result<String> {
    let mut args: Vec<&str> = vec!["docker", "run", "--rm", "--entrypoint", "sh"];
    let script = container_script(script);
    for mount in mounts {
        check_mountable(mount.split(':').next().unwrap_or(mount))?;
        args.extend(["-v", mount]);
    }
    args.extend([config.restic.image.as_str(), "-c", &script]);
    Ok(shell::join(&args))
}

pub fn sync(
    config: &Config,
    session: &Session,
    root: &str,
    snapshot_path: &str,
    dest: &Dest,
) -> Result<()> {
    session
        .run(&sync_command(config, root, snapshot_path, dest)?, &[], None)
        .map(|_| ())
}

pub struct Ctx<'a> {
    pub config: &'a Config,
    pub session: &'a Session,
    pub arcadia: ArcadiaFile,
    /// Empty in standard mode
    pub compose: Compose,
}

impl<'a> Ctx<'a> {
    pub fn load(config: &'a Config, session: &'a Session) -> Result<Ctx<'a>> {
        let path = format!("{}/config.yml", config.dir());
        let contents = session
            .run(&format!("cat {}", quote(&path)), &[], None)
            .with_context(|| format!("cannot read {path} on the Arcadia host"))?;
        let arcadia = arcadia_config::parse(&contents)?;
        let compose = match config.arcadia.mode {
            Mode::Docker => Compose::load(config, session)?,
            Mode::Standard => Compose::default(),
        };
        Ok(Ctx {
            config,
            session,
            arcadia,
            compose,
        })
    }

    pub fn run(&self, script: &str, secrets: &[Secret]) -> Result<String> {
        self.session.run(script, secrets, None)
    }

    /// Docker volume name of the compose volume `key`
    pub fn volume(&self, key: &str) -> Result<String> {
        self.compose
            .volumes
            .get(key)
            .cloned()
            .with_context(|| format!("docker compose declares no volume '{key}'"))
    }

    pub fn sync(&self, root: &str, snapshot_path: &str, dest: &Dest) -> Result<()> {
        sync(self.config, self.session, root, snapshot_path, dest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::test_config;

    #[test]
    fn parses_running_services() {
        assert_eq!(
            parse_running("db\nredis\n\n chevereto_database \n"),
            ["chevereto_database", "db", "redis"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        );
    }

    #[test]
    fn a_service_is_running_stopped_or_never_created() {
        let set = |names: &[&str]| names.iter().map(|s| s.to_string()).collect();
        let compose = Compose {
            running: set(&["db"]),
            existing: set(&["db", "chevereto_database"]),
            volumes: Default::default(),
        };
        assert_eq!(compose.state("db"), ServiceState::Running);
        assert_eq!(compose.state("chevereto_database"), ServiceState::Stopped);
        assert_eq!(compose.state("ergo_database"), ServiceState::Absent);
    }

    #[test]
    fn resolves_volume_names_from_compose_config() {
        let json = r#"{"name":"arcadia","services":{},"volumes":{
            "db_data":{"name":"arcadia_db_data"},
            "external_one":{"name":"shared","external":true},
            "unnamed":{}}}"#;
        let volumes = parse_volumes(json).unwrap();
        assert_eq!(volumes["db_data"], "arcadia_db_data");
        assert_eq!(volumes["external_one"], "shared");
        assert_eq!(volumes["unnamed"], "unnamed");
        assert!(parse_volumes(r#"{"services":{}}"#).unwrap().is_empty());
    }

    #[test]
    fn compose_commands_run_in_the_arcadia_dir() {
        assert_eq!(
            in_dir("/opt/my arcadia", &compose(&["ps", "--services"])),
            "cd '/opt/my arcadia' && docker compose ps --services"
        );
    }

    #[test]
    fn binary_runner_syncs_with_a_host_shell() {
        let config = test_config("standard", "binary");
        let command = sync_command(
            &config,
            "/w/restore",
            "/var/www/images",
            &Dest::Path("/var/www/images".into()),
        )
        .unwrap();
        assert_eq!(
            command,
            crate::shell::sync_script("/w/restore/var/www/images", "/var/www/images")
        );
        assert!(sync_command(&config, "/w/restore", "/x", &Dest::Volume("v".into())).is_err());
    }

    #[test]
    fn docker_runner_syncs_volumes_in_a_root_container() {
        let config = test_config("docker", "docker");
        let command = sync_command(
            &config,
            "/w/restore",
            "/arcadia/chevereto/files",
            &Dest::Volume("arcadia_chevereto_storage".into()),
        )
        .unwrap();
        assert!(command.starts_with(
            "docker run --rm --entrypoint sh -v /w/restore:/restore:ro -v arcadia_chevereto_storage:/target restic/restic:0.19.1 -c "
        ), "{command}");
        assert!(
            command.contains("/restore/arcadia/chevereto/files"),
            "{command}"
        );
    }

    #[test]
    fn docker_runner_syncs_host_paths_through_their_parent() {
        let config = test_config("docker", "docker");
        let command = sync_command(
            &config,
            "/w/restore",
            "/arcadia/config/config.yml",
            &Dest::Path("/opt/arcadia/config.yml".into()),
        )
        .unwrap();
        assert!(
            command.starts_with("mkdir -p /opt/arcadia && docker run"),
            "{command}"
        );
        assert!(command.contains("-v /opt/arcadia:/target "), "{command}");
        assert!(command.contains("/target/config.yml"), "{command}");
    }

    #[test]
    fn docker_scripts_stop_at_the_first_failure() {
        let config = test_config("docker", "docker");
        let command = sync_command(&config, "/w/restore", "/x", &Dest::Volume("v".into())).unwrap();
        assert!(command.contains("-c 'set -eu\n"), "{command}");
        let script = container_script("false\necho reached");
        let output = std::process::Command::new("sh")
            .args(["-c", &script])
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(!String::from_utf8_lossy(&output.stdout).contains("reached"));
    }

    #[test]
    fn docker_cannot_mount_paths_with_colons() {
        let config = test_config("docker", "docker");
        assert!(sync_command(
            &config,
            "/w/restore",
            "/a",
            &Dest::Path("/opt/a:b/c".into())
        )
        .is_err());
    }
}
