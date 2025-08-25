use sqlx::{postgres::PgPoolOptions, PgPool};
use std::borrow::{Borrow, BorrowMut};

pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    pub async fn try_new(db_uri: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(db_uri)
            .await
            .expect("Error building a connection pool");

        Ok(Self(pool))
    }

    pub fn with_pg_pool(pool: PgPool) -> Self {
        Self(pool)
    }
}

impl Borrow<PgPool> for ConnectionPool {
    fn borrow(&self) -> &PgPool {
        &self.0
    }
}

impl BorrowMut<PgPool> for ConnectionPool {
    fn borrow_mut(&mut self) -> &mut PgPool {
        &mut self.0
    }
}
