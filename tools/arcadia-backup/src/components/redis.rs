use super::{Component, Include, Restored, Source, Staged};
use crate::arcadia_config::Redis as RedisConfig;
use crate::config::Mode;
use crate::ctx::{compose, in_dir, root_container, Ctx};
use crate::shell::{self, quote, Secret};
use anyhow::{Context, Result};

pub struct Redis;

pub const DUMP: &str = "redis.rdb";

const CONTAINER_DUMP: &str =
    "redis-cli --rdb /tmp/arcadia-backup.rdb >&2 && cat /tmp/arcadia-backup.rdb && rm -f /tmp/arcadia-backup.rdb";

/// `redis-cli --rdb` streams a fresh snapshot from the running server, no downtime
pub fn dump_script(mode: Mode, dir: &str, redis: &RedisConfig, output: &str) -> String {
    let auth = !redis.password.is_empty();
    match mode {
        Mode::Docker => {
            let inner = if auth {
                format!("IFS= read -r REDISCLI_AUTH && export REDISCLI_AUTH && {CONTAINER_DUMP}")
            } else {
                CONTAINER_DUMP.to_string()
            };
            let exec = compose(&["exec", "-T", "redis", "sh", "-c", &inner]);
            // printf is a builtin: the password goes to the container on stdin, in no argv
            let pipeline = if auth {
                format!("printf '%s\\n' \"$REDISCLI_AUTH\" | {exec}")
            } else {
                exec
            };
            in_dir(dir, &format!("{pipeline} > {}", quote(output)))
        }
        Mode::Standard => {
            let port = redis.port.to_string();
            format!(
                "{} >&2",
                shell::join(&["redis-cli", "-h", &redis.host, "-p", &port, "--rdb", output])
            )
        }
    }
}

pub fn secrets(redis: &RedisConfig) -> Vec<Secret<'_>> {
    if redis.password.is_empty() {
        vec![]
    } else {
        vec![Secret {
            name: "REDISCLI_AUTH",
            value: &redis.password,
        }]
    }
}

impl Component for Redis {
    fn name(&self) -> &'static str {
        "redis"
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
                &ctx.arcadia.redis,
                &output,
            ),
            &secrets(&ctx.arcadia.redis),
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
        let dir = ctx.config.dir();
        match ctx.config.arcadia.mode {
            Mode::Docker => {
                let volume = ctx.volume("redis_data")?;
                let snapshot_path = restored.snapshot_path(DUMP)?;
                ctx.run(&in_dir(dir, &compose(&["stop", "redis"])), &[])?;
                let script = format!(
                    "cp {} /data/dump.rdb && chown \"$(stat -c %u:%g /data)\" /data/dump.rdb && rm -rf /data/appendonlydir",
                    quote(&format!("/restore{snapshot_path}"))
                );
                let mounts = [
                    format!("{}:/restore:ro", restored.root),
                    format!("{volume}:/data"),
                ];
                let copy = root_container(ctx.config, &mounts, &script)
                    .and_then(|command| ctx.run(&command, &[]));
                if let Err(error) = copy {
                    // do not leave redis down
                    let _ = ctx.run(&in_dir(dir, &compose(&["up", "-d", "redis"])), &[]);
                    return Err(error);
                }
                ctx.run(&in_dir(dir, &compose(&["up", "-d", "redis"])), &[])?;
            }
            Mode::Standard => {
                // the operator stopped redis before the restore, it loads the file when started again
                let rdb_path = ctx
                    .config
                    .standard
                    .as_ref()
                    .and_then(|standard| standard.redis.as_ref())
                    .map(|redis| redis.rdb_path.as_str())
                    .context("set standard.redis.rdb_path to restore redis")?;
                ctx.run(
                    &format!(
                        "cp {} {}",
                        quote(&restored.host_path(DUMP)?),
                        quote(rdb_path)
                    ),
                    &[],
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn redis(password: &str) -> RedisConfig {
        RedisConfig {
            host: "127.0.0.1".into(),
            port: 6379,
            password: password.into(),
        }
    }

    #[test]
    fn redis_without_password_sends_no_auth() {
        let script = dump_script(Mode::Docker, "/opt/arcadia", &redis(""), "/s/redis.rdb");
        assert!(!script.contains("REDISCLI_AUTH"), "{script}");
        assert!(
            script.starts_with("cd /opt/arcadia && docker compose exec -T redis sh -c"),
            "{script}"
        );
        assert!(script.ends_with("> /s/redis.rdb"), "{script}");
        assert!(secrets(&redis("")).is_empty());
    }

    #[test]
    fn docker_password_reaches_the_container_on_stdin() {
        let script = dump_script(Mode::Docker, "/opt/arcadia", &redis("pw"), "/s/redis.rdb");
        assert!(script.starts_with("cd /opt/arcadia && printf '%s\\n' \"$REDISCLI_AUTH\" | docker compose exec -T redis sh -c"), "{script}");
        assert!(script.contains("IFS= read -r REDISCLI_AUTH"), "{script}");
        assert!(!script.contains("pw "), "{script}");
    }

    #[test]
    fn standard_dump_uses_the_host_client() {
        assert_eq!(
            dump_script(Mode::Standard, "/srv/arcadia", &redis("pw"), "/s/redis.rdb"),
            "redis-cli -h 127.0.0.1 -p 6379 --rdb /s/redis.rdb >&2"
        );
        assert_eq!(secrets(&redis("pw"))[0].name, "REDISCLI_AUTH");
    }
}
