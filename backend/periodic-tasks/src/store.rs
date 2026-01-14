use crate::env::{formula_to_sql, Env};
use arcadia_storage::connection_pool::ConnectionPool;
use envconfig::Envconfig;
use std::sync::Arc;

pub struct Store {
    pub env: Env,
    pub pool: Arc<ConnectionPool>,
}

impl Store {
    pub async fn new() -> Self {
        let mut env = Env::init_from_env().unwrap();
        let pool = Arc::new(
            ConnectionPool::try_new(&env.database_url)
                .await
                .expect("db connection"),
        );
        env.periodic_tasks.bonus_points_formula =
            formula_to_sql(&env.periodic_tasks.bonus_points_formula)
                .expect("invalid bonus formula");

        Self { env, pool }
    }
}
