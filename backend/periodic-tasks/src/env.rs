use envconfig::Envconfig;

#[derive(Envconfig, Clone, Debug)]
pub struct Env {
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,
    #[envconfig(nested)]
    pub periodic_tasks: PeriodicTasksConfig,
}

#[derive(Envconfig, Clone, Debug)]
pub struct PeriodicTasksConfig {
    #[envconfig(from = "TASK_INTERVAL_USER_CLASS_CHANGES_SECONDS")]
    pub user_class_changes_seconds: u64,
    #[envconfig(from = "TASK_INTERVAL_SEEDTIME_AND_BONUS_POINTS_UPDATE_SECONDS")]
    pub seedtime_and_bonus_points_update_seconds: u64,
    #[envconfig(from = "BONUS_POINTS_FORMULA")]
    pub bonus_points_formula: String,
    #[envconfig(from = "TASK_INTERVAL_USER_TORRENT_STATS_UPDATE_SECONDS")]
    pub user_torrent_stats_update_seconds: u64,
}

/// Validates and converts a formula string to SQL expression.
/// Replaces variable names with their SQL equivalents:
/// - seedtime: total seed time in seconds
/// - seeders: replaced by the given `seeders_expression`
/// - size: torrent size in bytes
pub fn formula_to_sql(formula: &str, seeders_expression: &str) -> Result<String, &'static str> {
    for c in formula.chars() {
        if !c.is_alphanumeric()
            && c != ' '
            && c != '+'
            && c != '-'
            && c != '*'
            && c != '/'
            && c != '('
            && c != ')'
            && c != '.'
            && c != '_'
        {
            return Err("Formula contains invalid characters");
        }
    }

    let sql = formula
        .replace("seedtime", "ta.total_seed_time")
        .replace("seeders", seeders_expression)
        .replace("size", "t.size");

    Ok(sql)
}
