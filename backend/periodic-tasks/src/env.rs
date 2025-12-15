use envconfig::Envconfig;

#[derive(Envconfig, Clone)]
pub struct Env {
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,

    #[envconfig(nested)]
    pub tracker: TrackerConfig,

    #[envconfig(nested)]
    pub tasks: TasksConfig,
}

#[derive(Envconfig, Clone)]
pub struct TrackerConfig {
    #[envconfig(from = "ARCADIA_TRACKER_ANNOUNCE_INTERVAL")]
    pub announce_interval: u32,

    #[envconfig(
        from = "ARCADIA_TRACKER_ANNOUNCE_INTERVAL_GRACE_PERIOD",
        default = "300"
    )]
    pub announce_interval_grace_period: u32,
}

#[derive(Envconfig, Clone)]
pub struct TasksConfig {
    /// Cron schedule for updating torrent seeders/leechers count
    /// Default: every 5 minutes
    #[envconfig(
        from = "TASK_INTERVAL_UPDATE_SEEDERS_LEECHERS",
        default = "0 */5 * * * *"
    )]
    pub update_seeders_leechers: String,

    /// Cron schedule for removing inactive peers
    /// Default: every 2 minutes
    #[envconfig(
        from = "TASK_INTERVAL_REMOVE_INACTIVE_PEERS",
        default = "0 */2 * * * *"
    )]
    pub remove_inactive_peers: String,

    /// Cron schedule for expiring user warnings
    /// Default: every hour
    #[envconfig(from = "TASK_INTERVAL_EXPIRE_WARNINGS", default = "0 0 * * * *")]
    pub expire_warnings: String,

    /// Cron schedule for disabling inactive users
    /// Default: daily at 3am
    #[envconfig(from = "TASK_INTERVAL_DISABLE_INACTIVE_USERS", default = "0 0 3 * * *")]
    pub disable_inactive_users: String,

    /// Cron schedule for updating user seeding size
    /// Default: every 10 minutes
    #[envconfig(from = "TASK_INTERVAL_UPDATE_SEEDING_SIZE", default = "0 */10 * * * *")]
    pub update_seeding_size: String,

    /// Number of days after which a user is considered inactive
    #[envconfig(from = "INACTIVE_USER_DAYS", default = "90")]
    pub inactive_user_days: i32,
}
