use crate::config::Config;
use crate::restic::LocalRestic;
use anyhow::{bail, Result};

pub fn run(config: &Config) -> Result<()> {
    if config.restic.repository.join("config").exists() {
        bail!(
            "'{}' already holds a restic repository",
            config.restic.repository.display()
        );
    }
    LocalRestic::new(config).init()?;
    println!(
        "repository created in {}",
        config.restic.repository.display()
    );
    println!(
        "keep a copy of the password file somewhere safe: without it, the backups cannot be read"
    );
    Ok(())
}
