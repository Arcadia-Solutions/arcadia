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

/// Standard mode: whether the configured user may create databases. The restore drops the
/// database before creating it again, so this is checked before anything is modified.
pub fn create_privilege_script(db: &Database) -> String {
    let port = db.port.to_string();
    shell::join(&[
        "psql",
        "-h",
        &db.host,
        "-p",
        &port,
        "-U",
        &db.user,
        "-d",
        "postgres",
        "-Atc",
        "SELECT rolsuper OR rolcreatedb FROM pg_roles WHERE rolname = current_user",
    ])
}

/// What is wrong with the output of `create_privilege_script`, if anything
pub fn create_privilege_problem(user: &str, output: &str) -> Option<String> {
    if output.trim() == "t" {
        return None;
    }
    Some(format!(
        "the postgres user '{user}' cannot create databases, the restore would drop the database \
         and fail to create it again: run `ALTER ROLE {} CREATEDB;` as a superuser",
        identifier(user)
    ))
}

pub fn password_secret(db: &Database) -> Secret<'_> {
    Secret {
        name: "PGPASSWORD",
        value: &db.password,
    }
}

fn secrets<'c>(ctx: &'c Ctx) -> Vec<Secret<'c>> {
    match ctx.config.arcadia.mode {
        // the db container trusts its local connections
        Mode::Docker => vec![],
        Mode::Standard => vec![password_secret(&ctx.arcadia.database)],
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
    fn create_privilege_is_checked_on_the_maintenance_database() {
        let script = create_privilege_script(&database());
        assert_eq!(
            script,
            "psql -h 127.0.0.1 -p 5432 -U arcadia -d postgres -Atc \
             'SELECT rolsuper OR rolcreatedb FROM pg_roles WHERE rolname = current_user'"
        );
        assert!(!script.contains("s3cr"));
    }

    #[test]
    fn only_a_true_answer_allows_the_restore() {
        assert_eq!(create_privilege_problem("arcadia", "t\n"), None);
        for output in ["f\n", "", "garbage"] {
            let problem = create_privilege_problem("arcadia", output).unwrap();
            assert!(
                problem.contains(r#"ALTER ROLE "arcadia" CREATEDB"#),
                "{problem}"
            );
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
