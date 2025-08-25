use crate::env::Env;
use arcadia_storage::connection_pool::ConnectionPool;
use envconfig::Envconfig;
use std::sync::Arc;

pub struct Store {
    pub env: Env,
    pub pool: Arc<ConnectionPool>,
}

impl Store {
    pub async fn new() -> Self {
        let env = Env::init_from_env().unwrap();
        let pool = Arc::new(
            ConnectionPool::try_new(&env.database_url)
                .await
                .expect("db connection"),
        );

        Self { env, pool }
    }
}
