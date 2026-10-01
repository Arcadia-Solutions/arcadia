use crate::config::Config;
use crate::restic::{sort_latest_first, LocalRestic, Snapshot};
use anyhow::Result;

pub fn format_table(snapshots: &[Snapshot]) -> String {
    let mut table = format!("{:>3}  {:<8}  {:<25}  components\n", "#", "id", "date");
    for (index, snapshot) in sort_latest_first(snapshots).into_iter().enumerate() {
        let components: Vec<&str> = snapshot
            .tags
            .iter()
            .filter_map(|tag| tag.strip_prefix("component:"))
            .collect();
        table.push_str(&format!(
            "{index:>3}  {:<8}  {:<25}  {}\n",
            snapshot.short_id,
            snapshot.time.format("%Y-%m-%d %H:%M:%S %z").to_string(),
            components.join(", ")
        ));
    }
    table
}

pub fn run(config: &Config) -> Result<()> {
    let snapshots = LocalRestic::new(config).snapshots(&config.host_name())?;
    print!("{}", format_table(&snapshots));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_lists_latest_first_with_components() {
        let snapshot = |id: &str, time: &str, tags: &[&str]| Snapshot {
            id: format!("{id}00000000"),
            short_id: id.to_string(),
            time: chrono::DateTime::parse_from_rfc3339(time).unwrap(),
            hostname: "prod".into(),
            tags: tags.iter().map(|t| t.to_string()).collect(),
        };
        let table = format_table(&[
            snapshot(
                "aaaaaaaa",
                "2026-09-29T03:00:00+00:00",
                &["arcadia", "component:postgres"],
            ),
            snapshot(
                "bbbbbbbb",
                "2026-09-30T03:00:00+00:00",
                &[
                    "arcadia",
                    "mode:docker",
                    "component:postgres",
                    "component:redis",
                ],
            ),
        ]);
        assert_eq!(
            table,
            "  #  id        date                       components\n  \
               0  bbbbbbbb  2026-09-30 03:00:00 +0000  postgres, redis\n  \
               1  aaaaaaaa  2026-09-29 03:00:00 +0000  postgres\n"
        );
    }
}
