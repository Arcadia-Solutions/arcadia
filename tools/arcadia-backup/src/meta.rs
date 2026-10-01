//! meta.json, written in every snapshot: what it holds and where.

use crate::config::{Mode, Runner};
use anyhow::{bail, Context, Result};
use chrono::SecondsFormat;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
    pub format_version: u32,
    pub created_at: String,
    pub tool_version: String,
    pub mode: Mode,
    pub runner: Runner,
    /// `git rev-parse HEAD` of the Arcadia checkout, `unknown` when it is not a git checkout
    pub arcadia_git_commit: String,
    /// Component name → what it stored in the snapshot
    pub components: BTreeMap<String, Vec<Item>>,
    /// Optional files that did not exist at backup time, as `<component>/<key>`
    #[serde(default)]
    pub missing: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    /// Identifies the item inside its component, e.g. `config.yml` or `postgres.sql`
    pub key: String,
    /// Absolute path of the item inside the snapshot
    pub snapshot_path: String,
}

impl Meta {
    pub fn new(mode: Mode, runner: Runner, arcadia_git_commit: String) -> Meta {
        Meta {
            format_version: FORMAT_VERSION,
            created_at: chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            tool_version: env!("CARGO_PKG_VERSION").to_string(),
            mode,
            runner,
            arcadia_git_commit,
            components: BTreeMap::new(),
            missing: Vec::new(),
        }
    }

    pub fn from_json(bytes: &[u8]) -> Result<Meta> {
        let value: serde_json::Value =
            serde_json::from_slice(bytes).context("meta.json is not valid json")?;
        let version = value
            .get("format_version")
            .and_then(|v| v.as_u64())
            .context("meta.json has no format_version")?;
        if version != u64::from(FORMAT_VERSION) {
            bail!("the snapshot was made by another arcadia-backup version (meta.json format {version}), use a version that supports it");
        }
        serde_json::from_value(value).context("cannot read meta.json")
    }

    /// Fails when the snapshot cannot be restored on this target, returns the warnings to show otherwise
    pub fn check_target(&self, mode: Mode, git_commit: &str) -> Result<Vec<String>> {
        if self.mode != mode {
            bail!(
                "the snapshot was made in {} mode, the configuration is in {} mode",
                self.mode.as_str(),
                mode.as_str()
            );
        }
        let mut warnings = Vec::new();
        if self.arcadia_git_commit != git_commit {
            warnings.push(format!(
                "the snapshot was made with Arcadia at commit {}, the Arcadia host is at commit {}: \
                 the database schema may not match the code",
                self.arcadia_git_commit, git_commit
            ));
        }
        Ok(warnings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta() -> Meta {
        let mut meta = Meta::new(Mode::Docker, Runner::Docker, "37d64014".to_string());
        meta.components.insert(
            "postgres".to_string(),
            vec![Item {
                key: "postgres.sql".into(),
                snapshot_path: "/arcadia/postgres/postgres.sql".into(),
            }],
        );
        meta.missing.push("config/compose.override.yml".to_string());
        meta
    }

    #[test]
    fn round_trips_through_json() {
        let meta = meta();
        let json = serde_json::to_vec_pretty(&meta).unwrap();
        assert_eq!(Meta::from_json(&json).unwrap(), meta);
        assert_eq!(meta.format_version, FORMAT_VERSION);
        assert_eq!(meta.tool_version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn newer_formats_are_rejected_with_a_clear_message() {
        let error = Meta::from_json(br#"{"format_version": 2, "something": "new"}"#)
            .unwrap_err()
            .to_string();
        assert!(error.contains("format 2"), "{error}");
    }

    #[test]
    fn mode_must_match() {
        let error = meta()
            .check_target(Mode::Standard, "37d64014")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("docker mode") && error.contains("standard mode"),
            "{error}"
        );
    }

    #[test]
    fn another_commit_is_only_a_warning() {
        assert!(meta()
            .check_target(Mode::Docker, "37d64014")
            .unwrap()
            .is_empty());
        let warnings = meta().check_target(Mode::Docker, "985f4534").unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("37d64014") && warnings[0].contains("985f4534"));
    }
}
