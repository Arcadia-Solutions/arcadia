//! Files of the Arcadia checkout: configuration and custom content.

use super::{Component, Include, Restored, Source, Staged};
use crate::config::Config;
use crate::ctx::{self, Ctx, Dest};
use crate::shell::{self, quote};
use crate::ssh::Session;
use anyhow::Result;

#[derive(Clone, Copy)]
pub struct RepoFiles {
    pub name: &'static str,
    /// Relative to arcadia.dir, all optional
    pub paths: &'static [&'static str],
}

pub const CONFIG: RepoFiles = RepoFiles {
    name: "config",
    paths: &[
        "config.yml",
        "compose.override.yml",
        ".env",
        "ergo/ergo-conf.yaml",
        "ergo/ergo.motd",
        "kiwiirc/config.json",
    ],
};

pub const CUSTOM_CONTENT: RepoFiles = RepoFiles {
    name: "custom_content",
    paths: &[
        "frontend/public/home",
        "frontend/public/favicon.ico",
        "frontend/public/default_user_avatar.png",
        "frontend/public/bonus_points_icon.png",
        "frontend/public/custom_unauth.css",
        "frontend/public/custom_unauth.js",
    ],
};

pub fn existence_script(dir: &str, paths: &[&str]) -> String {
    format!(
        "cd {} && for p in {}; do if [ -e \"$p\" ]; then echo \"$p\"; fi; done",
        quote(dir),
        shell::join(paths)
    )
}

/// (present, missing), in the order of `paths`
pub fn split_existing(paths: &[&str], output: &str) -> (Vec<String>, Vec<String>) {
    let existing: Vec<&str> = output.lines().map(str::trim).collect();
    paths
        .iter()
        .map(|path| path.to_string())
        .partition(|path| existing.contains(&path.as_str()))
}

impl RepoFiles {
    /// Restores the items into arcadia.dir. Usable before a `Ctx` exists, the config component
    /// being what makes `Ctx::load` possible on a new host.
    pub fn restore_with(
        &self,
        config: &Config,
        session: &Session,
        restored: &Restored,
    ) -> Result<()> {
        for item in restored.items {
            let dest = Dest::Path(format!("{}/{}", config.dir(), item.key));
            ctx::sync(config, session, restored.root, &item.snapshot_path, &dest)?;
        }
        Ok(())
    }
}

impl Component for RepoFiles {
    fn name(&self) -> &'static str {
        self.name
    }

    fn is_present(&self, _: &Ctx) -> Result<bool> {
        Ok(true)
    }

    fn stage(&self, ctx: &Ctx, _staging: &str) -> Result<Staged> {
        let dir = ctx.config.dir();
        let output = ctx.run(&existence_script(dir, self.paths), &[])?;
        let (present, missing) = split_existing(self.paths, &output);
        Ok(Staged {
            includes: present
                .into_iter()
                .map(|path| Include {
                    source: Source::Path(format!("{dir}/{path}")),
                    key: path,
                })
                .collect(),
            missing: missing
                .into_iter()
                .map(|path| format!("{}/{path}", self.name))
                .collect(),
        })
    }

    fn restore(&self, ctx: &Ctx, restored: &Restored) -> Result<()> {
        self.restore_with(ctx.config, ctx.session, restored)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn existence_script_lists_the_files_that_exist() {
        assert_eq!(
            existence_script("/opt/arcadia", &["config.yml", ".env"]),
            "cd /opt/arcadia && for p in config.yml .env; do if [ -e \"$p\" ]; then echo \"$p\"; fi; done"
        );
    }

    #[test]
    fn split_existing_separates_present_and_missing() {
        let (present, missing) = split_existing(CONFIG.paths, "config.yml\nergo/ergo-conf.yaml\n");
        assert_eq!(present, vec!["config.yml", "ergo/ergo-conf.yaml"]);
        assert_eq!(
            missing,
            vec![
                "compose.override.yml",
                ".env",
                "ergo/ergo.motd",
                "kiwiirc/config.json"
            ]
        );
    }

    #[test]
    fn custom_content_covers_the_git_ignored_frontend_files() {
        assert!(CUSTOM_CONTENT.paths.contains(&"frontend/public/home"));
        assert!(CUSTOM_CONTENT
            .paths
            .contains(&"frontend/public/custom_unauth.css"));
        assert!(CUSTOM_CONTENT
            .paths
            .contains(&"frontend/public/custom_unauth.js"));
    }
}
