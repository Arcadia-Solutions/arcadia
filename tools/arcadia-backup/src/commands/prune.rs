use crate::config::Config;
use crate::restic::{self, LocalRestic};
use anyhow::{bail, Result};

pub fn run(config: &Config) -> Result<()> {
    let local = LocalRestic::new(config);
    let host = config.host_name();
    let blockers =
        restic::future_snapshots(&local.snapshots(&host)?, chrono::Utc::now().fixed_offset());
    if !blockers.is_empty() {
        bail!(
            "the retention policy was not applied, it would keep bogus snapshots over the real history: {}; inspect them with `arcadia-backup snapshots`, and remove them with restic if they are not yours",
            blockers.join("; ")
        );
    }
    local.forget(&host, &config.retention)
}
