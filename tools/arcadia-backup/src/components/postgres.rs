use super::{Component, Include, Restored, Source, Staged};
use crate::arcadia_config::Database;
use crate::config::Mode;
use crate::ctx::{compose, in_dir, Ctx};
use crate::shell::{self, quote, Secret};
use anyhow::Result;

pub struct Postgres;

pub const DUMP: &str = "postgres.sql";

pub fn identifier(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

/// Plain format and uncompressed on purpose: restic chunks it by content, so a few changed rows
/// only add a few chunks
pub fn dump_script(mode: Mode, dir: &str, db: &Database, output: &str) -> String {
    let options = ["--no-owner", "--no-privileges"];
    match mode {
        Mode::Docker => {
            let mut args = vec![
                "exec", "-T", "db", "pg_dump", "-U", &db.user, "-d", &db.name,
            ];
            args.extend(options);
            in_dir(dir, &format!("{} > {}", compose(&args), quote(output)))
        }
        Mode::Standard => {
            let port = db.port.to_string();
            let mut args = vec![
                "pg_dump", "-h", &db.host, "-p", &port, "-U", &db.user, "-d", &db.name,
            ];
            args.extend(options);
            format!("{} > {}", shell::join(&args), quote(output))
        }
    }
}

pub fn restore_script(mode: Mode, dir: &str, db: &Database, file: &str) -> String {
    let drop = format!(
        "DROP DATABASE IF EXISTS {} WITH (FORCE)",
        identifier(&db.name)
    );
    let create = format!(
        "CREATE DATABASE {} OWNER {}",
        identifier(&db.name),
        identifier(&db.user)
    );
    let recreate = [
        "-d",
        "postgres",
        "-v",
        "ON_ERROR_STOP=1",
        "-c",
        &drop,
        "-c",
        &create,
    ];
    let load = ["-d", &db.name, "-v", "ON_ERROR_STOP=1", "-1", "-q"];
    match mode {
        Mode::Docker => {
            let psql = |rest: &[&str]| {
                let mut args = vec!["exec", "-T", "db", "psql", "-U", &db.user];
                args.extend(rest);
                compose(&args)
            };
            in_dir(
                dir,
                &format!("{} && {} < {}", psql(&recreate), psql(&load), quote(file)),
            )
        }
        Mode::Standard => {
            let port = db.port.to_string();
            let psql = |rest: &[&str]| {
                let mut args = vec!["psql", "-h", &db.host, "-p", &port, "-U", &db.user];
                args.extend(rest);
                shell::join(&args)
            };
            format!("{} && {} < {}", psql(&recreate), psql(&load), quote(file))
        }
    }
}

fn secrets<'c>(ctx: &'c Ctx) -> Vec<Secret<'c>> {
    match ctx.config.arcadia.mode {
        // the db container trusts its local connections
        Mode::Docker => vec![],
        Mode::Standard => vec![Secret {
            name: "PGPASSWORD",
            value: &ctx.arcadia.database.password,
        }],
    }
}

impl Component for Postgres {
    fn name(&self) -> &'static str {
        "postgres"
    }

    fn is_present(&self, _: &Ctx) -> Result<bool> {
        Ok(true)
    }

    fn stage(&self, ctx: &Ctx, staging: &str) -> Result<Staged> {
        let output = format!("{staging}/{DUMP}");
        ctx.run(
            &dump_script(
                ctx.config.arcadia.mode,
                ctx.config.dir(),
                &ctx.arcadia.database,
                &output,
            ),
            &secrets(ctx),
        )?;
        Ok(Staged {
            includes: vec![Include {
                key: DUMP.to_string(),
                source: Source::Path(output),
            }],
            missing: vec![],
        })
    }

    fn restore(&self, ctx: &Ctx, restored: &Restored) -> Result<()> {
        let file = restored.host_path(DUMP)?;
        ctx.run(
            &restore_script(
                ctx.config.arcadia.mode,
                ctx.config.dir(),
                &ctx.arcadia.database,
                &file,
            ),
            &secrets(ctx),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn database() -> Database {
        Database {
            host: "127.0.0.1".into(),
            port: 5432,
            user: "arcadia".into(),
            password: "s3cr'et".into(),
            name: "arcadia".into(),
        }
    }

    #[test]
    fn docker_dump_runs_in_the_db_service() {
        assert_eq!(
            dump_script(Mode::Docker, "/opt/arcadia", &database(), "/opt/arcadia/.arcadia-backup/staging/postgres.sql"),
            "cd /opt/arcadia && docker compose exec -T db pg_dump -U arcadia -d arcadia --no-owner --no-privileges \
             > /opt/arcadia/.arcadia-backup/staging/postgres.sql"
        );
    }

    #[test]
    fn standard_dump_connects_by_host_without_the_password_in_the_command() {
        let script = dump_script(
            Mode::Standard,
            "/srv/arcadia",
            &database(),
            "/s/postgres.sql",
        );
        assert_eq!(script, "pg_dump -h 127.0.0.1 -p 5432 -U arcadia -d arcadia --no-owner --no-privileges > /s/postgres.sql");
        assert!(!script.contains("s3cr"));
    }

    #[test]
    fn restore_recreates_the_database_then_loads_in_one_transaction() {
        let script = restore_script(
            Mode::Docker,
            "/opt/arcadia",
            &database(),
            "/w/r/postgres.sql",
        );
        assert!(script.contains("-d postgres -v ON_ERROR_STOP=1 -c 'DROP DATABASE IF EXISTS \"arcadia\" WITH (FORCE)' -c 'CREATE DATABASE \"arcadia\" OWNER \"arcadia\"'"), "{script}");
        assert!(script.ends_with("docker compose exec -T db psql -U arcadia -d arcadia -v ON_ERROR_STOP=1 -1 -q < /w/r/postgres.sql"), "{script}");
    }

    #[test]
    fn identifiers_are_quoted() {
        assert_eq!(identifier("we\"ird"), "\"we\"\"ird\"");
    }
}
