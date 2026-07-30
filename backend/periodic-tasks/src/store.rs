use crate::config::{formula_to_sql, PeriodicTasksConfig};
use arcadia_storage::connection_pool::ConnectionPool;
use std::sync::Arc;

pub struct Store {
    pub config: PeriodicTasksConfig,
    pub pool: Arc<ConnectionPool>,
}

impl Store {
    /// The pool and the configuration are the ones of the service running the tasks.
    pub fn new(pool: Arc<ConnectionPool>, mut config: PeriodicTasksConfig) -> Self {
        config.bonus_points_formula = formula_to_sql(&config.bonus_points_formula, "t.seeders")
            .expect("invalid bonus formula");

        Self { config, pool }
    }
}
