//! What is backed up. Each component knows how to stage itself for a backup and how to restore
//! itself, so both directions cannot drift apart.

pub mod extra;
pub mod files;
pub mod mariadb;
pub mod postgres;
pub mod redis;

use crate::config::Runner;
use crate::ctx::Ctx;
use crate::meta::Item;
use anyhow::{bail, Context, Result};

#[derive(Debug)]
pub enum Source {
    /// Absolute path on the Arcadia host
    Path(String),
    /// Docker volume name
    Volume(String),
}

impl Source {
    pub fn mount_source(&self) -> &str {
        match self {
            Source::Path(path) => path,
            Source::Volume(volume) => volume,
        }
    }
}

/// Something restic must include in the snapshot
#[derive(Debug)]
pub struct Include {
    pub key: String,
    pub source: Source,
}

#[derive(Debug, Default)]
pub struct Staged {
    pub includes: Vec<Include>,
    /// Optional things that do not exist, as `<component>/<key>`
    pub missing: Vec<String>,
}

/// A snapshot restored on the Arcadia host in `root`, and the items of one component
pub struct Restored<'a> {
    pub root: &'a str,
    pub items: &'a [Item],
}

impl Restored<'_> {
    pub fn snapshot_path(&self, key: &str) -> Result<&str> {
        self.items
            .iter()
            .find(|item| item.key == key)
            .map(|item| item.snapshot_path.as_str())
            .with_context(|| format!("the snapshot has no '{key}'"))
    }

    /// Where the item is on the Arcadia host once the snapshot is restored in `root`
    pub fn host_path(&self, key: &str) -> Result<String> {
        Ok(format!("{}{}", self.root, self.snapshot_path(key)?))
    }
}

pub trait Component {
    fn name(&self) -> &'static str;
    /// docker mode: its service is running; standard mode: it is configured
    fn is_present(&self, ctx: &Ctx) -> Result<bool>;
    /// Dumps what needs dumping into `staging`, returns what restic must include
    fn stage(&self, ctx: &Ctx, staging: &str) -> Result<Staged>;
    fn restore(&self, ctx: &Ctx, restored: &Restored) -> Result<()>;
}

/// Every component, in restore order
pub fn all() -> Vec<Box<dyn Component>> {
    vec![
        Box::new(files::CONFIG),
        Box::new(files::CUSTOM_CONTENT),
        Box::new(postgres::Postgres),
        Box::new(redis::Redis),
        Box::new(mariadb::CHEVERETO),
        Box::new(mariadb::ERGO),
        Box::new(extra::Extra),
    ]
}

pub fn names() -> Vec<&'static str> {
    all().iter().map(|component| component.name()).collect()
}

/// Where an include lands in the snapshot
pub fn snapshot_path(runner: Runner, component: &str, include: &Include) -> Result<String> {
    match runner {
        Runner::Docker => Ok(format!("/arcadia/{component}/{}", include.key)),
        Runner::Binary => match &include.source {
            Source::Path(path) => Ok(path.clone()),
            Source::Volume(volume) => {
                bail!("the volume '{volume}' can only be backed up with restic.runner: docker")
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_is_in_restore_order() {
        assert_eq!(
            names(),
            vec![
                "config",
                "custom_content",
                "postgres",
                "redis",
                "chevereto",
                "ergo",
                "extra"
            ]
        );
    }

    #[test]
    fn docker_snapshot_paths_are_under_arcadia() {
        let include = Include {
            key: "frontend/public/home".into(),
            source: Source::Path("/opt/arcadia/frontend/public/home".into()),
        };
        assert_eq!(
            snapshot_path(Runner::Docker, "custom_content", &include).unwrap(),
            "/arcadia/custom_content/frontend/public/home"
        );
        let volume = Include {
            key: "files".into(),
            source: Source::Volume("arcadia_ergo_data".into()),
        };
        assert_eq!(
            snapshot_path(Runner::Docker, "ergo", &volume).unwrap(),
            "/arcadia/ergo/files"
        );
    }

    #[test]
    fn binary_snapshot_paths_are_the_host_paths() {
        let include = Include {
            key: "files".into(),
            source: Source::Path("/var/lib/ergo".into()),
        };
        assert_eq!(
            snapshot_path(Runner::Binary, "ergo", &include).unwrap(),
            "/var/lib/ergo"
        );
        let volume = Include {
            key: "files".into(),
            source: Source::Volume("v".into()),
        };
        assert!(snapshot_path(Runner::Binary, "ergo", &volume).is_err());
    }

    #[test]
    fn restored_paths_are_under_the_restore_root() {
        let items = vec![Item {
            key: "postgres.sql".into(),
            snapshot_path: "/arcadia/postgres/postgres.sql".into(),
        }];
        let restored = Restored {
            root: "/w/restore",
            items: &items,
        };
        assert_eq!(
            restored.host_path("postgres.sql").unwrap(),
            "/w/restore/arcadia/postgres/postgres.sql"
        );
        assert!(restored.host_path("other").is_err());
    }
}
