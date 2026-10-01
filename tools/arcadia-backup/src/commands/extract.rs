use crate::config::Config;
use crate::meta::Meta;
use crate::restic::LocalRestic;
use crate::selector::Selector;
use anyhow::{bail, Context, Result};
use std::io::ErrorKind;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// Makes `path` an empty directory only its owner can enter, creating it when needed
pub fn prepare_target(path: &Path) -> Result<()> {
    match std::fs::read_dir(path) {
        Ok(mut entries) => {
            if entries.next().is_some() {
                bail!(not_empty(path));
            }
        }
        Err(error) if error.kind() == ErrorKind::NotFound => {
            std::fs::create_dir_all(path)
                .with_context(|| format!("cannot create '{}'", path.display()))?;
        }
        // a file, most likely
        Err(_) if path.exists() && !path.is_dir() => bail!(not_empty(path)),
        Err(error) => {
            return Err(error).with_context(|| format!("cannot read '{}'", path.display()));
        }
    }
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))
        .with_context(|| format!("cannot restrict '{}' to its owner", path.display()))
}

fn not_empty(path: &Path) -> String {
    format!(
        "'{}' exists and is not empty, choose a new or empty directory",
        path.display()
    )
}

/// One line per item of the snapshot: `<component>/<key>: <where it was extracted>`
pub fn item_lines(meta: &Meta, target: &Path) -> Vec<String> {
    meta.components
        .iter()
        .flat_map(|(component, items)| {
            items.iter().map(move |item| {
                let path = target.join(item.snapshot_path.trim_start_matches('/'));
                format!("{component}/{}: {}", item.key, path.display())
            })
        })
        .collect()
}

pub fn run(config: &Config, snapshot: &Selector, target: &Path) -> Result<()> {
    let local = LocalRestic::new(config);
    let snapshots = local.snapshots(&config.host_name())?;
    let snapshot = snapshot.resolve(&snapshots)?;
    prepare_target(target)?;
    eprintln!(
        "--> extracting snapshot {} into {}",
        snapshot.short_id,
        target.display()
    );
    local.restore_to(&snapshot.id, target)?;
    match local
        .meta_json(&snapshot.id, &config.work_dir())
        .and_then(|json| Meta::from_json(&json))
    {
        Ok(meta) => {
            for line in item_lines(&meta, target) {
                println!("{line}");
            }
        }
        Err(error) => println!(
            "the snapshot has no usable meta.json ({error:#}), look into {} yourself",
            target.display()
        ),
    }
    eprintln!(
        "warning: the extracted files are not encrypted and contain secrets (config.yml, database dumps): delete {} when you are done",
        target.display()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Mode, Runner};
    use crate::meta::Item;
    use std::os::unix::fs::PermissionsExt;

    fn mode_of(path: &Path) -> u32 {
        std::fs::metadata(path).unwrap().permissions().mode() & 0o777
    }

    #[test]
    fn creates_a_missing_target_with_mode_700() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("a/b");
        prepare_target(&target).unwrap();
        assert!(target.is_dir());
        assert_eq!(mode_of(&target), 0o700);
    }

    #[test]
    fn restricts_an_empty_target_to_mode_700() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o755)).unwrap();
        prepare_target(dir.path()).unwrap();
        assert_eq!(mode_of(dir.path()), 0o700);
    }

    #[test]
    fn refuses_a_non_empty_target_and_leaves_it_alone() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("x"), "x").unwrap();
        std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o755)).unwrap();
        let error = prepare_target(dir.path()).unwrap_err().to_string();
        assert!(
            error.contains("exists and is not empty, choose a new or empty directory"),
            "{error}"
        );
        assert!(error.contains(&dir.path().display().to_string()), "{error}");
        assert_eq!(mode_of(dir.path()), 0o755);
    }

    #[test]
    fn refuses_a_file_as_target() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("f");
        std::fs::write(&file, "x").unwrap();
        let error = prepare_target(&file).unwrap_err().to_string();
        assert!(error.contains("exists and is not empty"), "{error}");
    }

    #[test]
    fn lines_name_each_item_under_the_target() {
        let mut meta = Meta::new(Mode::Docker, Runner::Docker, "abc".into());
        meta.components.insert(
            "postgres".into(),
            vec![Item {
                key: "postgres.sql".into(),
                snapshot_path: "/arcadia/postgres/postgres.sql".into(),
            }],
        );
        meta.components.insert(
            "config".into(),
            vec![
                Item {
                    key: "config.yml".into(),
                    snapshot_path: "/arcadia/config/config.yml".into(),
                },
                Item {
                    key: "compose.yml".into(),
                    snapshot_path: "/arcadia/config/compose.yml".into(),
                },
            ],
        );
        assert_eq!(
            item_lines(&meta, Path::new("/tmp/x")),
            vec![
                "config/config.yml: /tmp/x/arcadia/config/config.yml",
                "config/compose.yml: /tmp/x/arcadia/config/compose.yml",
                "postgres/postgres.sql: /tmp/x/arcadia/postgres/postgres.sql",
            ]
        );
        assert_eq!(
            item_lines(&meta, Path::new("/tmp/x/"))[2],
            "postgres/postgres.sql: /tmp/x/arcadia/postgres/postgres.sql"
        );
    }
}
