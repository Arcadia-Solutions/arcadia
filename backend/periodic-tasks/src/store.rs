use crate::env::{formula_to_sql, Env};
use arcadia_storage::connection_pool::{ConnectionPool, TrackerConfig};
use envconfig::Envconfig;
use std::sync::Arc;

pub struct Store {
    pub env: Env,
    pub pool: Arc<ConnectionPool>,
}

impl Store {
    pub async fn new(tracker_config: TrackerConfig) -> Self {
        let mut env = Env::init_from_env().unwrap();
        let pool = Arc::new(
            ConnectionPool::try_new(&env.database_url, tracker_config)
                .await
                .expect("db connection"),
        );
        env.periodic_tasks.bonus_points_formula =
            formula_to_sql(&env.periodic_tasks.bonus_points_formula, "t.seeders")
                .expect("invalid bonus formula");

        Self { env, pool }
    }
}
