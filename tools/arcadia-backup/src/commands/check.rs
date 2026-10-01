use crate::config::Config;
use crate::restic::LocalRestic;
use anyhow::Result;

pub fn run(config: &Config, read_data_subset: Option<&str>) -> Result<()> {
    let subset = read_data_subset.or(config.restic.check.read_data_subset.as_deref());
    LocalRestic::new(config).check(subset)
}
