use crate::commands::{self, RemoteDirGuard};
use crate::components::{self, Source};
use crate::config::{Config, Runner};
use crate::ctx::Ctx;
use crate::meta::{Item, Meta};
use crate::restic::{self, LocalRestic, RemoteRestic, DOCKER_META_PATH};
use crate::shell::quote;
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    pub backups_since_check: u32,
}

/// Counts the backup, tells whether a check is due now
pub fn check_due(state: &mut State, every: u32) -> bool {
    if every == 0 {
        return false;
    }
    state.backups_since_check += 1;
    if state.backups_since_check >= every {
        state.backups_since_check = 0;
        true
    } else {
        false
    }
}

pub fn run(config: &Config) -> Result<()> {
    let password = config.read_password()?;
    let started = Instant::now();
    let local = LocalRestic::new(config);
    let host = config.host_name();
    let before = local.snapshots(&host)?;

    let summary = {
        let connection = commands::connect(config)?;
        let session = &connection.session;
        let ctx = Ctx::load(config, session)?;

        let staging = format!("{}/staging", config.work_dir());
        session.run(
            &format!("rm -rf {0} && mkdir -m 700 {0}", quote(&staging)),
            &[],
            None,
        )?;
        let _cleanup = RemoteDirGuard {
            session,
            path: staging.clone(),
        };

        let mut meta = Meta::new(
            config.arcadia.mode,
            config.restic.runner,
            commands::git_commit(config, session)?,
        );
        let mut entries: Vec<(Source, String)> = Vec::new();
        let mut tags = vec![
            restic::TAG.to_string(),
            format!("mode:{}", config.arcadia.mode.as_str()),
        ];

        for component in components::all() {
            let name = component.name();
            if !component.is_present(&ctx)? {
                eprintln!("--> {name}: not deployed, skipped");
                continue;
            }
            eprintln!("--> {name}");
            let staged = component
                .stage(&ctx, &staging)
                .with_context(|| format!("cannot back up {name}"))?;
            let mut items = Vec::new();
            for include in staged.includes {
                let snapshot_path =
                    components::snapshot_path(config.restic.runner, name, &include)?;
                items.push(Item {
                    key: include.key,
                    snapshot_path: snapshot_path.clone(),
                });
                entries.push((include.source, snapshot_path));
            }
            meta.components.insert(name.to_string(), items);
            meta.missing.extend(staged.missing);
            tags.push(format!("component:{name}"));
        }

        let meta_file = format!("{staging}/meta.json");
        session.run(
            &format!("cat > {}", quote(&meta_file)),
            &[],
            Some(&serde_json::to_vec_pretty(&meta)?),
        )?;
        let meta_snapshot_path = match config.restic.runner {
            Runner::Docker => DOCKER_META_PATH.to_string(),
            Runner::Binary => meta_file.clone(),
        };
        entries.push((Source::Path(meta_file), meta_snapshot_path));

        eprintln!("--> restic backup");
        let remote = RemoteRestic {
            config,
            port: session.remote_port,
        };
        let script = remote.backup_script(&entries, &host, &tags)?;
        let output = session.run(
            &script,
            &commands::restic_secrets(&password, &connection.server),
            None,
        )?;
        restic::parse_backup_summary(&output)?
    };

    let blockers = restic::retention_blockers(
        &before,
        &local.snapshots(&host)?,
        &summary.snapshot_id,
        chrono::Utc::now().fixed_offset(),
    );
    if !blockers.is_empty() {
        bail!(
            "snapshot {} was made, but the retention policy was skipped: {}; \
             the Arcadia host may have planted snapshots, inspect them with `arcadia-backup snapshots`",
            summary.snapshot_id,
            blockers.join("; ")
        );
    }
    eprintln!("--> applying the retention policy");
    local.forget(&host, &config.retention)?;

    let state_path = config.state_path();
    let mut state: State = std::fs::read(&state_path)
        .ok()
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default();
    if check_due(&mut state, config.restic.check.every_n_backups) {
        eprintln!("--> checking the repository");
        local.check(config.restic.check.read_data_subset.as_deref())?;
    }
    std::fs::write(&state_path, serde_json::to_vec(&state)?)
        .with_context(|| format!("cannot write {}", state_path.display()))?;

    println!(
        "snapshot {} data_added={} duration={}s",
        summary.snapshot_id,
        summary.data_added,
        started.elapsed().as_secs()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_runs_every_n_backups() {
        let mut state = State::default();
        let due: Vec<bool> = (0..6).map(|_| check_due(&mut state, 3)).collect();
        assert_eq!(due, vec![false, false, true, false, false, true]);
        assert!(!check_due(&mut State::default(), 0));
    }
}
