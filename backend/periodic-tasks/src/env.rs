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
    #[envconfig(from = "TASK_INTERVAL_USER_CLASS_CHANGES")]
    pub user_class_changes: String,
}
