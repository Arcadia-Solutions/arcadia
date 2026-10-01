//! The mariadb database and the files of chevereto and ergo.

use super::{Component, Include, Restored, Source, Staged};
use crate::config::{MariadbConnection, MariadbWithFiles, Mode, StandardConfig};
use crate::ctx::{compose, in_dir, Ctx, Dest, ServiceState};
use crate::shell::{self, quote};
use anyhow::{bail, Context, Result};

pub const DUMP: &str = "dump.sql";
pub const FILES: &str = "files";

pub struct Mariadb {
    pub name: &'static str,
    pub db_service: &'static str,
    /// Compose volume key of the files
    pub volume: &'static str,
    pub standard: fn(&StandardConfig) -> Option<&MariadbWithFiles>,
}

fn standard_chevereto(standard: &StandardConfig) -> Option<&MariadbWithFiles> {
    standard.chevereto.as_ref()
}

fn standard_ergo(standard: &StandardConfig) -> Option<&MariadbWithFiles> {
    standard.ergo.as_ref()
}

pub const CHEVERETO: Mariadb = Mariadb {
    name: "chevereto",
    db_service: "chevereto_database",
    volume: "chevereto_storage",
    standard: standard_chevereto,
};

pub const ERGO: Mariadb = Mariadb {
    name: "ergo",
    db_service: "ergo_database",
    volume: "ergo_data",
    standard: standard_ergo,
};

// Run inside the database container, with its own credentials
const DOCKER_DUMP: &str = r#"export MYSQL_PWD="$MYSQL_PASSWORD"; exec mariadb-dump -u"$MYSQL_USER" --single-transaction "$MYSQL_DATABASE""#;
const DOCKER_RESTORE: &str = r#"export MYSQL_PWD="$MYSQL_PASSWORD"; mariadb -u"$MYSQL_USER" -e "DROP DATABASE IF EXISTS \`$MYSQL_DATABASE\`; CREATE DATABASE \`$MYSQL_DATABASE\`" && exec mariadb -u"$MYSQL_USER" "$MYSQL_DATABASE""#;

pub fn docker_dump_script(dir: &str, service: &str, output: &str) -> String {
    in_dir(
        dir,
        &format!(
            "{} > {}",
            compose(&["exec", "-T", service, "sh", "-c", DOCKER_DUMP]),
            quote(output)
        ),
    )
}

pub fn docker_restore_script(dir: &str, service: &str, file: &str) -> String {
    in_dir(
        dir,
        &format!(
            "{} < {}",
            compose(&["exec", "-T", service, "sh", "-c", DOCKER_RESTORE]),
            quote(file)
        ),
    )
}

fn password_prefix(connection: &MariadbConnection) -> String {
    format!(
        "MYSQL_PWD=\"$(cat {})\"; export MYSQL_PWD; ",
        quote(&connection.password_file)
    )
}

fn client(program: &str, connection: &MariadbConnection) -> Vec<String> {
    [
        program,
        "-h",
        &connection.host,
        "-P",
        &connection.port.to_string(),
        "-u",
        &connection.user,
    ]
    .iter()
    .map(|arg| arg.to_string())
    .collect()
}

pub fn standard_dump_script(connection: &MariadbConnection, output: &str) -> String {
    let mut args = client("mariadb-dump", connection);
    args.extend(["--single-transaction".to_string(), connection.name.clone()]);
    format!(
        "{}{} > {}",
        password_prefix(connection),
        shell::join(&args),
        quote(output)
    )
}

pub fn standard_restore_script(connection: &MariadbConnection, file: &str) -> String {
    let name = format!("`{}`", connection.name.replace('`', "``"));
    let mut recreate = client("mariadb", connection);
    recreate.extend([
        "-e".to_string(),
        format!("DROP DATABASE IF EXISTS {name}; CREATE DATABASE {name}"),
    ]);
    let mut load = client("mariadb", connection);
    load.push(connection.name.clone());
    format!(
        "{}{} && {} < {}",
        password_prefix(connection),
        shell::join(&recreate),
        shell::join(&load),
        quote(file)
    )
}

/// A stopped database must not make the backup silently skip the service
pub fn docker_presence(service: &str, state: ServiceState) -> Result<bool> {
    match state {
        ServiceState::Running => Ok(true),
        ServiceState::Absent => Ok(false),
        ServiceState::Stopped => bail!(
            "{service} exists but is not running: start it, or remove the service to stop backing it up"
        ),
    }
}

impl Mariadb {
    fn standard_config<'c>(&self, ctx: &'c Ctx) -> Option<&'c MariadbWithFiles> {
        ctx.config
            .standard
            .as_ref()
            .and_then(|standard| (self.standard)(standard))
    }
}

impl Component for Mariadb {
    fn name(&self) -> &'static str {
        self.name
    }

    fn is_present(&self, ctx: &Ctx) -> Result<bool> {
        Ok(match ctx.config.arcadia.mode {
            Mode::Docker => docker_presence(self.db_service, ctx.compose.state(self.db_service))?,
            Mode::Standard => self.standard_config(ctx).is_some(),
        })
    }

    fn stage(&self, ctx: &Ctx, staging: &str) -> Result<Staged> {
        let output = format!("{staging}/{}.sql", self.name);
        let files = match ctx.config.arcadia.mode {
            Mode::Docker => {
                ctx.run(
                    &docker_dump_script(ctx.config.dir(), self.db_service, &output),
                    &[],
                )?;
                Source::Volume(ctx.volume(self.volume)?)
            }
            Mode::Standard => {
                let config = self.standard_config(ctx).context("not configured")?;
                ctx.run(&standard_dump_script(&config.database, &output), &[])?;
                Source::Path(config.dir.clone())
            }
        };
        Ok(Staged {
            includes: vec![
                Include {
                    key: DUMP.to_string(),
                    source: Source::Path(output),
                },
                Include {
                    key: FILES.to_string(),
                    source: files,
                },
            ],
            missing: vec![],
        })
    }

    fn restore(&self, ctx: &Ctx, restored: &Restored) -> Result<()> {
        let dump = restored.host_path(DUMP)?;
        let files = restored.snapshot_path(FILES)?;
        match ctx.config.arcadia.mode {
            Mode::Docker => {
                ctx.run(
                    &docker_restore_script(ctx.config.dir(), self.db_service, &dump),
                    &[],
                )?;
                ctx.sync(
                    restored.root,
                    files,
                    &Dest::Volume(ctx.volume(self.volume)?),
                )
            }
            Mode::Standard => {
                let config = self
                    .standard_config(ctx)
                    .context("not configured in the standard section")?;
                ctx.run(&standard_restore_script(&config.database, &dump), &[])?;
                ctx.sync(restored.root, files, &Dest::Path(config.dir.clone()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn connection() -> MariadbConnection {
        MariadbConnection {
            host: "127.0.0.1".into(),
            port: 3307,
            user: "ergo".into(),
            password_file: "/etc/arcadia/ergo.pass".into(),
            name: "ergo_history".into(),
        }
    }

    #[test]
    fn docker_dump_uses_the_container_credentials() {
        let script = docker_dump_script("/opt/arcadia", "ergo_database", "/s/ergo.sql");
        assert!(
            script.starts_with("cd /opt/arcadia && docker compose exec -T ergo_database sh -c "),
            "{script}"
        );
        assert!(
            script.contains("export MYSQL_PWD=\"$MYSQL_PASSWORD\""),
            "{script}"
        );
        assert!(script.contains("--single-transaction"), "{script}");
        assert!(script.ends_with("> /s/ergo.sql"), "{script}");
    }

    #[test]
    fn docker_restore_recreates_the_database() {
        let script = docker_restore_script("/opt/arcadia", "chevereto_database", "/w/r/dump.sql");
        assert!(script.contains("DROP DATABASE IF EXISTS"), "{script}");
        assert!(script.ends_with("< /w/r/dump.sql"), "{script}");
    }

    #[test]
    fn standard_scripts_read_the_password_file_on_the_arcadia_host() {
        let dump = standard_dump_script(&connection(), "/s/ergo.sql");
        assert_eq!(
            dump,
            "MYSQL_PWD=\"$(cat /etc/arcadia/ergo.pass)\"; export MYSQL_PWD; \
             mariadb-dump -h 127.0.0.1 -P 3307 -u ergo --single-transaction ergo_history > /s/ergo.sql"
        );
        let restore = standard_restore_script(&connection(), "/w/r/ergo.sql");
        assert!(
            restore.contains(
                "-e 'DROP DATABASE IF EXISTS `ergo_history`; CREATE DATABASE `ergo_history`'"
            ),
            "{restore}"
        );
        assert!(
            restore.ends_with("mariadb -h 127.0.0.1 -P 3307 -u ergo ergo_history < /w/r/ergo.sql"),
            "{restore}"
        );
    }

    #[test]
    fn a_stopped_database_fails_instead_of_being_skipped() {
        assert!(docker_presence("ergo_database", ServiceState::Running).unwrap());
        assert!(!docker_presence("ergo_database", ServiceState::Absent).unwrap());
        let error = docker_presence("chevereto_database", ServiceState::Stopped)
            .unwrap_err()
            .to_string();
        assert!(
            error.contains(
                "chevereto_database exists but is not running: start it, or remove the service"
            ),
            "{error}"
        );
    }

    #[test]
    fn components_use_their_own_services() {
        assert_eq!(
            (CHEVERETO.name, CHEVERETO.db_service, CHEVERETO.volume),
            ("chevereto", "chevereto_database", "chevereto_storage")
        );
        assert_eq!(
            (ERGO.name, ERGO.db_service, ERGO.volume),
            ("ergo", "ergo_database", "ergo_data")
        );
    }
}
