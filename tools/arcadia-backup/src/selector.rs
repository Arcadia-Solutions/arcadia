//! Which snapshot to restore: `latest`, how many backups before the latest, or an id.

use crate::restic::{sort_latest_first, Snapshot};
use anyhow::{anyhow, bail, Result};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Selector {
    Latest,
    /// `n` backups before the latest one, `0` being the latest
    Ago(usize),
    /// Prefix of a snapshot id, at least 8 hexadecimal characters
    Id(String),
}

impl FromStr for Selector {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, String> {
        if value == "latest" {
            return Ok(Selector::Latest);
        }
        // Ids are at least 8 characters long, so short numbers are unambiguous
        if !value.is_empty() && value.len() <= 6 && value.bytes().all(|b| b.is_ascii_digit()) {
            return value
                .parse()
                .map(Selector::Ago)
                .map_err(|error| format!("{error}"));
        }
        if value.len() >= 8 && value.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Ok(Selector::Id(value.to_ascii_lowercase()));
        }
        Err(format!(
            "'{value}' is not a snapshot: use `latest`, a number of backups before the latest (e.g. `1`), \
             or a snapshot id (at least 8 hexadecimal characters)"
        ))
    }
}

impl Selector {
    pub fn resolve<'a>(&self, snapshots: &'a [Snapshot]) -> Result<&'a Snapshot> {
        let sorted = sort_latest_first(snapshots);
        match self {
            Selector::Latest => sorted
                .first()
                .copied()
                .ok_or_else(|| anyhow!("the repository has no snapshot of this host")),
            Selector::Ago(n) => sorted.get(*n).copied().ok_or_else(|| {
                anyhow!(
                    "there are only {} snapshots, there is none {n} backups before the latest",
                    sorted.len()
                )
            }),
            Selector::Id(prefix) => {
                let matching: Vec<&Snapshot> = sorted
                    .into_iter()
                    .filter(|s| s.id.starts_with(prefix.as_str()))
                    .collect();
                match matching.as_slice() {
                    [snapshot] => Ok(snapshot),
                    [] => bail!("no snapshot id starts with '{prefix}'"),
                    _ => bail!(
                        "'{prefix}' matches {} snapshots, give more characters",
                        matching.len()
                    ),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(id: &str, time: &str) -> Snapshot {
        Snapshot {
            id: id.to_string(),
            short_id: id[..8].to_string(),
            time: chrono::DateTime::parse_from_rfc3339(time).unwrap(),
            hostname: "prod".to_string(),
            tags: vec![],
        }
    }

    fn snapshots() -> Vec<Snapshot> {
        // deliberately not sorted, one of them in another offset
        vec![
            snapshot("bbbbbbbb22222222", "2026-09-29T03:00:00+00:00"),
            snapshot("cccccccc33333333", "2026-09-30T05:00:00+02:00"),
            snapshot("aaaaaaaa11111111", "2026-09-28T03:00:00+00:00"),
            snapshot("aaaaaaab44444444", "2026-09-27T03:00:00+00:00"),
        ]
    }

    #[test]
    fn parses_selectors() {
        assert_eq!("latest".parse(), Ok(Selector::Latest));
        assert_eq!("0".parse(), Ok(Selector::Ago(0)));
        assert_eq!("12".parse(), Ok(Selector::Ago(12)));
        assert_eq!("AAAAAAAA".parse(), Ok(Selector::Id("aaaaaaaa".to_string())));
        assert!("abc".parse::<Selector>().is_err());
        assert!("12345678"
            .parse::<Selector>()
            .is_ok_and(|s| s == Selector::Id("12345678".to_string())));
        assert!("-1".parse::<Selector>().is_err());
    }

    #[test]
    fn latest_is_the_most_recent_whatever_the_offset() {
        assert_eq!(
            Selector::Latest.resolve(&snapshots()).unwrap().id,
            "cccccccc33333333"
        );
    }

    #[test]
    fn ago_counts_backups_before_the_latest() {
        let snapshots = snapshots();
        assert_eq!(
            Selector::Ago(0).resolve(&snapshots).unwrap().id,
            "cccccccc33333333"
        );
        assert_eq!(
            Selector::Ago(1).resolve(&snapshots).unwrap().id,
            "bbbbbbbb22222222"
        );
        assert_eq!(
            Selector::Ago(3).resolve(&snapshots).unwrap().id,
            "aaaaaaab44444444"
        );
        let error = Selector::Ago(4)
            .resolve(&snapshots)
            .unwrap_err()
            .to_string();
        assert!(error.contains("only 4 snapshots"), "{error}");
    }

    #[test]
    fn ids_match_by_unique_prefix() {
        let snapshots = snapshots();
        assert_eq!(
            Selector::Id("bbbbbbbb".into())
                .resolve(&snapshots)
                .unwrap()
                .id,
            "bbbbbbbb22222222"
        );
        assert_eq!(
            Selector::Id("aaaaaaaa1".into())
                .resolve(&snapshots)
                .unwrap()
                .id,
            "aaaaaaaa11111111"
        );
        let unknown = Selector::Id("dddddddd".into())
            .resolve(&snapshots)
            .unwrap_err()
            .to_string();
        assert!(
            unknown.contains("no snapshot id starts with 'dddddddd'"),
            "{unknown}"
        );
    }

    #[test]
    fn ambiguous_ids_are_rejected() {
        let snapshots = vec![
            snapshot("abcdef0011111111", "2026-09-29T03:00:00+00:00"),
            snapshot("abcdef0022222222", "2026-09-28T03:00:00+00:00"),
        ];
        let error = Selector::Id("abcdef00".into())
            .resolve(&snapshots)
            .unwrap_err()
            .to_string();
        assert!(error.contains("matches 2 snapshots"), "{error}");
    }

    #[test]
    fn empty_repository() {
        let error = Selector::Latest.resolve(&[]).unwrap_err().to_string();
        assert!(error.contains("no snapshot"), "{error}");
    }
}
