use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The configuration of arcadia-backup, read on the backup host.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub ssh: SshConfig,
    pub arcadia: ArcadiaConfig,
    pub restic: ResticConfig,
    pub retention: Retention,
    #[serde(default)]
    pub extra: Extra,
    pub standard: Option<StandardConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SshConfig {
    /// `user@host` or a host alias of `~/.ssh/config`
    pub target: String,
    /// Extra arguments handed to every ssh invocation, before the target
    #[serde(default)]
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Docker,
    Standard,
}

impl Mode {
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Docker => "docker",
            Mode::Standard => "standard",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArcadiaConfig {
    /// Repository checkout on the Arcadia host
    pub dir: String,
    pub mode: Mode,
    /// Host name recorded in the snapshots, defaults to the host of `ssh.target`
    pub name: Option<String>,
    /// Where the dumps are staged and the snapshots restored, defaults to `<dir>/.arcadia-backup`
    pub work_dir: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Runner {
    /// restic runs from its docker image on the Arcadia host
    Docker,
    /// restic runs from a binary installed on the Arcadia host
    Binary,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResticConfig {
    /// Local path of the repository, on the backup host
    pub repository: PathBuf,
    pub password_file: PathBuf,
    pub runner: Runner,
    #[serde(default = "default_image")]
    pub image: String,
    /// restic on the backup host
    #[serde(default = "default_restic")]
    pub binary: String,
    /// restic on the Arcadia host, for `runner: binary`
    #[serde(default = "default_restic")]
    pub remote_binary: String,
    #[serde(default = "default_rest_server")]
    pub rest_server: String,
    #[serde(default)]
    pub check: CheckConfig,
}

fn default_image() -> String {
    "restic/restic:0.19.1".to_string()
}

fn default_restic() -> String {
    "restic".to_string()
}

fn default_rest_server() -> String {
    "rest-server".to_string()
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckConfig {
    /// Run `restic check` every n backups, 0 disables it
    #[serde(default)]
    pub every_n_backups: u32,
    /// e.g. `5%`, handed to `restic check --read-data-subset`
    pub read_data_subset: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Retention {
    pub keep_last: Option<u32>,
    pub keep_hourly: Option<u32>,
    pub keep_daily: Option<u32>,
    pub keep_weekly: Option<u32>,
    pub keep_monthly: Option<u32>,
    pub keep_yearly: Option<u32>,
}

impl Retention {
    fn entries(&self) -> [(&'static str, Option<u32>); 6] {
        [
            ("--keep-last", self.keep_last),
            ("--keep-hourly", self.keep_hourly),
            ("--keep-daily", self.keep_daily),
            ("--keep-weekly", self.keep_weekly),
            ("--keep-monthly", self.keep_monthly),
            ("--keep-yearly", self.keep_yearly),
        ]
    }

    pub fn is_empty(&self) -> bool {
        self.entries().iter().all(|(_, value)| value.is_none())
    }

    /// The `restic forget` flags of the policy
    pub fn forget_args(&self) -> Vec<String> {
        self.entries()
            .into_iter()
            .filter_map(|(flag, value)| value.map(|value| [flag.to_string(), value.to_string()]))
            .flatten()
            .collect()
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Extra {
    /// Absolute paths on the Arcadia host
    #[serde(default)]
    pub paths: Vec<String>,
    /// Docker volume names, as listed by `docker volume ls`
    #[serde(default)]
    pub volumes: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StandardConfig {
    pub chevereto: Option<MariadbWithFiles>,
    pub ergo: Option<MariadbWithFiles>,
    pub redis: Option<StandardRedis>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MariadbWithFiles {
    pub database: MariadbConnection,
    /// Directory holding the files of the service
    pub dir: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MariadbConnection {
    #[serde(default = "default_mariadb_host")]
    pub host: String,
    #[serde(default = "default_mariadb_port")]
    pub port: u16,
    pub user: String,
    /// Path on the Arcadia host of a file holding the password
    pub password_file: String,
    pub name: String,
}

fn default_mariadb_host() -> String {
    "127.0.0.1".to_string()
}

fn default_mariadb_port() -> u16 {
    3306
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StandardRedis {
    /// The `dump.rdb` redis loads at startup, replaced on restore
    pub rdb_path: String,
}

impl Config {
    pub fn parse(contents: &str) -> Result<Config> {
        let mut config: Config = serde_norway::from_str(contents)?;
        config.restic.repository = expand_tilde(&config.restic.repository);
        config.restic.password_file = expand_tilde(&config.restic.password_file);
        trim_slashes(&mut config.arcadia.dir);
        if let Some(work_dir) = &mut config.arcadia.work_dir {
            trim_slashes(work_dir);
        }
        config.extra.paths.iter_mut().for_each(trim_slashes);
        config.validate()?;
        Ok(config)
    }

    /// Reports every problem at once, before anything connects
    fn validate(&self) -> Result<()> {
        let mut errors: Vec<String> = Vec::new();
        if self.ssh.target.trim().is_empty() {
            errors.push("ssh.target must not be empty".to_string());
        }
        if self.retention.is_empty() {
            errors.push("retention: at least one keep_* key is required".to_string());
        }
        for (flag, value) in self.retention.entries() {
            if value == Some(0) {
                let key = flag.trim_start_matches("--").replace('-', "_");
                errors.push(format!("retention: {key} must be greater than 0"));
            }
        }

        let mut absolute: Vec<(&str, &str)> = vec![("arcadia.dir", self.arcadia.dir.as_str())];
        if let Some(work_dir) = &self.arcadia.work_dir {
            absolute.push(("arcadia.work_dir", work_dir));
        }
        for path in &self.extra.paths {
            absolute.push(("extra.paths", path));
        }
        if let Some(standard) = &self.standard {
            for (key, component) in [
                ("standard.chevereto", &standard.chevereto),
                ("standard.ergo", &standard.ergo),
            ] {
                if let Some(component) = component {
                    absolute.push((key, &component.dir));
                    absolute.push((key, &component.database.password_file));
                }
            }
            if let Some(redis) = &standard.redis {
                absolute.push(("standard.redis.rdb_path", &redis.rdb_path));
            }
        }
        for path in &self.extra.paths {
            if path.starts_with("/volume/") {
                errors.push(format!("extra.paths: '{path}' cannot be under /volume/"));
            }
        }
        for (key, path) in absolute {
            if !path.starts_with('/') {
                errors.push(format!("{key}: '{path}' must be an absolute path"));
            }
        }
        errors.extend(extra_path_problems(
            self.dir(),
            &self.work_dir(),
            &self.extra.paths,
        ));

        match self.arcadia.mode {
            Mode::Docker => {
                if self.standard.is_some() {
                    errors.push("standard: only allowed with arcadia.mode: standard".to_string());
                }
                if self.restic.runner == Runner::Binary {
                    errors.push(
                        "restic.runner: binary cannot read the docker volumes, use runner: docker with arcadia.mode: docker"
                            .to_string(),
                    );
                }
            }
            Mode::Standard => {
                if !self.extra.volumes.is_empty() {
                    errors
                        .push("extra.volumes: only allowed with arcadia.mode: docker".to_string());
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            bail!("invalid configuration:\n  - {}", errors.join("\n  - "))
        }
    }

    /// `arcadia.dir` without its trailing slash
    pub fn dir(&self) -> &str {
        self.arcadia.dir.trim_end_matches('/')
    }

    pub fn work_dir(&self) -> String {
        match &self.arcadia.work_dir {
            Some(work_dir) => work_dir.trim_end_matches('/').to_string(),
            None => format!("{}/.arcadia-backup", self.dir()),
        }
    }

    pub fn host_name(&self) -> String {
        match &self.arcadia.name {
            Some(name) => name.clone(),
            None => self
                .ssh
                .target
                .rsplit('@')
                .next()
                .unwrap_or(&self.ssh.target)
                .to_string(),
        }
    }

    pub fn read_password(&self) -> Result<String> {
        let path = &self.restic.password_file;
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("cannot read restic.password_file '{}'", path.display()))?;
        // the way restic reads RESTIC_PASSWORD_FILE (BOM stripped, TrimSpace), so that the local
        // restic and the remote one, which gets the value itself, use the same password
        let password = contents.trim_start_matches('\u{feff}').trim();
        if password.is_empty() || password.contains('\n') {
            bail!(
                "restic.password_file '{}' must hold the password on a single line",
                path.display()
            );
        }
        Ok(password.to_string())
    }

    /// Small state file next to the repository, counting the backups since the last check
    pub fn state_path(&self) -> PathBuf {
        let mut path = self.restic.repository.clone().into_os_string();
        path.push(".arcadia-backup-state");
        path.into()
    }
}

/// `/srv/x/` → `/srv/x`; a path made of slashes only is left alone
fn trim_slashes(path: &mut String) {
    let trimmed = path.trim_end_matches('/');
    if !trimmed.is_empty() {
        path.truncate(trimmed.len());
    }
}

/// `path` is `ancestor` or lies inside it
fn is_within(path: &str, ancestor: &str) -> bool {
    let ancestor = ancestor.trim_end_matches('/');
    path.trim_end_matches('/') == ancestor
        || path
            .strip_prefix(ancestor)
            .is_some_and(|rest| rest.starts_with('/'))
}

/// Extra paths that would back up (and restore over) what the tool itself manages
pub fn extra_path_problems(dir: &str, work_dir: &str, paths: &[String]) -> Vec<String> {
    let mut problems = Vec::new();
    for path in paths {
        if is_within(dir, path) {
            problems.push(format!(
                "extra.paths: '{path}' is or contains arcadia.dir '{dir}', which is already backed up by its components"
            ));
        } else if is_within(work_dir, path) || is_within(path, work_dir) {
            problems.push(format!(
                "extra.paths: '{path}' overlaps the work directory '{work_dir}', where dumps are staged and snapshots restored"
            ));
        }
    }
    problems
}

pub fn load(path: &Path) -> Result<Config> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("cannot read the configuration file '{}'", path.display()))?;
    Config::parse(&contents)
        .with_context(|| format!("in the configuration file '{}'", path.display()))
}

pub fn default_path() -> Result<PathBuf> {
    let home = std::env::var_os("HOME").context("HOME is not set, pass --config")?;
    Ok(PathBuf::from(home).join(".config/arcadia-backup/arcadia-backup.yml"))
}

fn expand_tilde(path: &Path) -> PathBuf {
    match (path.strip_prefix("~"), std::env::var_os("HOME")) {
        (Ok(rest), Some(home)) => PathBuf::from(home).join(rest),
        _ => path.to_path_buf(),
    }
}

/// A valid configuration for the unit tests of the other modules
#[cfg(test)]
pub fn test_config(mode: &str, runner: &str) -> Config {
    Config::parse(&format!(
        "ssh:\n  target: backup@arcadia.example.com\narcadia:\n  dir: /opt/arcadia\n  mode: {mode}\n  name: prod\n\
         restic:\n  repository: /srv/backups/arcadia\n  password_file: /etc/arcadia-backup/password\n  runner: {runner}\n\
         retention:\n  keep_last: 3\n"
    ))
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    const DOCKER: &str = "\
ssh:
  target: backup@arcadia.example.com
arcadia:
  dir: /opt/arcadia
  mode: docker
restic:
  repository: /srv/backups/arcadia
  password_file: /etc/arcadia-backup/password
  runner: docker
retention:
  keep_daily: 7
  keep_weekly: 4
";

    const STANDARD: &str = "\
ssh:
  target: arcadia.example.com
  options: [\"-p\", \"2222\"]
arcadia:
  dir: /srv/arcadia/
  mode: standard
  name: prod
restic:
  repository: ~/backups/arcadia
  password_file: /etc/arcadia-backup/password
  runner: binary
retention:
  keep_last: 30
standard:
  chevereto:
    database: { user: chevereto, password_file: /etc/arcadia/chevereto.pass, name: chevereto }
    dir: /var/www/chevereto/images
  redis:
    rdb_path: /var/lib/redis/dump.rdb
";

    fn error_of(contents: &str) -> String {
        format!("{:#}", Config::parse(contents).unwrap_err())
    }

    #[test]
    fn parses_a_docker_configuration_with_defaults() {
        let config = Config::parse(DOCKER).unwrap();
        assert_eq!(config.arcadia.mode, Mode::Docker);
        assert_eq!(config.restic.runner, Runner::Docker);
        assert_eq!(config.restic.image, "restic/restic:0.19.1");
        assert_eq!(config.restic.binary, "restic");
        assert_eq!(config.restic.remote_binary, "restic");
        assert_eq!(config.restic.rest_server, "rest-server");
        assert_eq!(config.restic.check.every_n_backups, 0);
        assert_eq!(config.dir(), "/opt/arcadia");
        assert_eq!(config.work_dir(), "/opt/arcadia/.arcadia-backup");
        assert_eq!(config.host_name(), "arcadia.example.com");
        assert_eq!(
            config.state_path(),
            PathBuf::from("/srv/backups/arcadia.arcadia-backup-state")
        );
    }

    #[test]
    fn parses_a_standard_configuration() {
        let config = Config::parse(STANDARD).unwrap();
        assert_eq!(config.dir(), "/srv/arcadia");
        assert_eq!(config.host_name(), "prod");
        assert_eq!(config.ssh.options, vec!["-p", "2222"]);
        let chevereto = config
            .standard
            .as_ref()
            .unwrap()
            .chevereto
            .as_ref()
            .unwrap();
        assert_eq!(chevereto.database.host, "127.0.0.1");
        assert_eq!(chevereto.database.port, 3306);
        let home = std::env::var("HOME").unwrap();
        assert_eq!(
            config.restic.repository,
            PathBuf::from(home).join("backups/arcadia")
        );
    }

    #[test]
    fn retention_is_required() {
        let contents = DOCKER.replace(
            "retention:\n  keep_daily: 7\n  keep_weekly: 4\n",
            "retention: {}\n",
        );
        assert!(error_of(&contents).contains("retention: at least one keep_* key is required"));
    }

    #[test]
    fn retention_values_must_be_positive() {
        let contents = DOCKER.replace("keep_daily: 7", "keep_daily: 0");
        assert!(error_of(&contents).contains("retention: keep_daily must be greater than 0"));
    }

    #[test]
    fn forget_args_follow_restic_flag_order() {
        let retention = Retention {
            keep_last: Some(30),
            keep_monthly: Some(6),
            ..Default::default()
        };
        assert_eq!(
            retention.forget_args(),
            vec!["--keep-last", "30", "--keep-monthly", "6"]
        );
    }

    #[test]
    fn docker_mode_rejects_standard_section_and_binary_runner_together() {
        let contents = format!(
            "{}standard:\n  redis:\n    rdb_path: /var/lib/redis/dump.rdb\n",
            DOCKER.replace("runner: docker", "runner: binary")
        );
        let error = error_of(&contents);
        assert!(
            error.contains("standard: only allowed with arcadia.mode: standard"),
            "{error}"
        );
        assert!(
            error.contains("restic.runner: binary cannot read the docker volumes"),
            "{error}"
        );
    }

    #[test]
    fn standard_mode_rejects_volumes() {
        let contents = format!("{STANDARD}extra:\n  volumes: [caddy_data]\n");
        assert!(
            error_of(&contents).contains("extra.volumes: only allowed with arcadia.mode: docker")
        );
    }

    #[test]
    fn extra_paths_cannot_collide_with_volume_keys() {
        let contents = format!("{DOCKER}extra:\n  paths: [/volume/x]\n");
        assert!(error_of(&contents).contains("extra.paths: '/volume/x' cannot be under /volume/"));
    }

    #[test]
    fn paths_must_be_absolute() {
        let contents = DOCKER.replace("dir: /opt/arcadia", "dir: opt/arcadia");
        assert!(error_of(&contents).contains("arcadia.dir: 'opt/arcadia' must be an absolute path"));
    }

    #[test]
    fn extra_paths_cannot_overlap_the_directories_of_the_tool() {
        for path in [
            "/opt/arcadia",
            "/opt",
            "/",
            "/opt/arcadia/.arcadia-backup/x",
            "/opt/arcadia/.arcadia-backup",
        ] {
            let contents = format!("{DOCKER}extra:\n  paths: [{path}]\n");
            let error = error_of(&contents);
            assert!(
                error.contains(&format!("extra.paths: '{path}'")),
                "{path}: {error}"
            );
        }
        let contents = DOCKER.replace(
            "  mode: docker\n",
            "  mode: docker\n  work_dir: /var/lib/ab/\n",
        );
        let error = error_of(&format!(
            "{contents}extra:\n  paths: [/var/lib/ab/staging, /var/lib]\n"
        ));
        assert!(
            error.contains("'/var/lib/ab/staging'") && error.contains("'/var/lib'"),
            "{error}"
        );
        // siblings and other places are fine
        let contents = format!(
            "{DOCKER}extra:\n  paths: [/opt/arcadia-plugin, /opt/arcadia/plugin, /srv/x]\n"
        );
        Config::parse(&contents).unwrap();
    }

    #[test]
    fn trailing_slashes_are_normalised() {
        let contents = DOCKER.replace("dir: /opt/arcadia", "dir: /opt/arcadia//");
        let config = Config::parse(&format!("{contents}extra:\n  paths: [/srv/x/]\n")).unwrap();
        assert_eq!(config.arcadia.dir, "/opt/arcadia");
        assert_eq!(config.extra.paths, vec!["/srv/x"]);
    }

    #[test]
    fn unknown_keys_are_rejected() {
        let contents = format!("{DOCKER}typo: 1\n");
        assert!(error_of(&contents).contains("unknown field `typo`"));
    }

    #[test]
    fn host_name_without_user() {
        let contents = DOCKER.replace("backup@arcadia.example.com", "arcadia");
        assert_eq!(Config::parse(&contents).unwrap().host_name(), "arcadia");
    }

    #[test]
    fn test_config_builds_valid_configurations() {
        assert_eq!(test_config("docker", "docker").arcadia.mode, Mode::Docker);
        assert_eq!(
            test_config("standard", "binary").restic.runner,
            Runner::Binary
        );
    }

    #[test]
    fn password_is_read_like_restic_reads_it() {
        let dir = tempfile::tempdir().unwrap();
        let mut config = test_config("docker", "docker");
        config.restic.password_file = dir.path().join("password");
        for contents in ["secret\n", "  secret \t\r\n", "\u{feff}secret\n", "secret"] {
            std::fs::write(&config.restic.password_file, contents).unwrap();
            assert_eq!(config.read_password().unwrap(), "secret", "{contents:?}");
        }
        for contents in ["", " \n", "first\nsecond\n"] {
            std::fs::write(&config.restic.password_file, contents).unwrap();
            assert!(config.read_password().is_err(), "{contents:?}");
        }
    }

    #[test]
    fn the_example_configuration_is_valid() {
        let config = Config::parse(include_str!("../arcadia-backup.example.yml")).unwrap();
        assert_eq!(config.arcadia.mode, Mode::Docker);
        assert!(!config.retention.is_empty());
    }
}
