use super::{Component, Include, Restored, Source, Staged};
use crate::config::Extra as ExtraConfig;
use crate::ctx::{Ctx, Dest};
use crate::shell;
use anyhow::{bail, Context, Result};

pub struct Extra;

pub fn includes(extra: &ExtraConfig) -> Vec<Include> {
    let paths = extra.paths.iter().map(|path| Include {
        key: path.trim_start_matches('/').to_string(),
        source: Source::Path(path.clone()),
    });
    let volumes = extra.volumes.iter().map(|volume| Include {
        key: format!("volume/{volume}"),
        source: Source::Volume(volume.clone()),
    });
    paths.chain(volumes).collect()
}

pub fn destination(key: &str) -> Dest {
    match key.strip_prefix("volume/") {
        Some(volume) => Dest::Volume(volume.to_string()),
        None => Dest::Path(format!("/{key}")),
    }
}

/// The configured paths that the existence check did not print
pub fn missing_paths<'a>(paths: &[&'a str], output: &str) -> Vec<&'a str> {
    let existing: Vec<&str> = output.lines().collect();
    paths
        .iter()
        .copied()
        .filter(|path| !existing.contains(path))
        .collect()
}

impl Component for Extra {
    fn name(&self) -> &'static str {
        "extra"
    }

    fn is_present(&self, ctx: &Ctx) -> Result<bool> {
        Ok(!ctx.config.extra.paths.is_empty() || !ctx.config.extra.volumes.is_empty())
    }

    fn stage(&self, ctx: &Ctx, _staging: &str) -> Result<Staged> {
        let paths: Vec<&str> = ctx.config.extra.paths.iter().map(String::as_str).collect();
        if !paths.is_empty() {
            let existing = ctx.run(
                &format!(
                    "for p in {}; do if [ -e \"$p\" ]; then echo \"$p\"; fi; done",
                    shell::join(&paths)
                ),
                &[],
            )?;
            let missing = missing_paths(&paths, &existing);
            if !missing.is_empty() {
                bail!(
                    "extra.paths: {} does not exist on the Arcadia host",
                    missing.join(", ")
                );
            }
        }
        for volume in &ctx.config.extra.volumes {
            ctx.run(
                &format!("docker volume inspect {} >/dev/null", shell::quote(volume)),
                &[],
            )
            .with_context(|| {
                format!("extra.volumes: the docker volume '{volume}' does not exist")
            })?;
        }
        Ok(Staged {
            includes: includes(&ctx.config.extra),
            missing: vec![],
        })
    }

    fn restore(&self, ctx: &Ctx, restored: &Restored) -> Result<()> {
        for item in restored.items {
            ctx.sync(restored.root, &item.snapshot_path, &destination(&item.key))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_stable_and_distinct() {
        let extra = crate::config::Extra {
            paths: vec!["/srv/plugin/data".into()],
            volumes: vec!["caddy_data".into()],
        };
        let includes = includes(&extra);
        assert_eq!(includes[0].key, "srv/plugin/data");
        assert!(matches!(&includes[0].source, Source::Path(p) if p == "/srv/plugin/data"));
        assert_eq!(includes[1].key, "volume/caddy_data");
        assert!(matches!(&includes[1].source, Source::Volume(v) if v == "caddy_data"));
    }

    #[test]
    fn configured_paths_that_do_not_exist_are_reported() {
        assert_eq!(missing_paths(&["/a", "/b", "/c"], "/a\n/c\n"), vec!["/b"]);
        assert!(missing_paths(&["/a"], "/a\n").is_empty());
    }

    #[test]
    fn keys_map_back_to_their_destination() {
        assert!(matches!(destination("volume/caddy_data"), Dest::Volume(v) if v == "caddy_data"));
        assert!(matches!(destination("srv/plugin/data"), Dest::Path(p) if p == "/srv/plugin/data"));
    }
}
