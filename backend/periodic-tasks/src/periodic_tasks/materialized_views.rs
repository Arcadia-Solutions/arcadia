use arcadia_storage::connection_pool::ConnectionPool;
use sqlx::PgPool;
use std::borrow::Borrow;
use std::sync::Arc;

pub async fn refresh_title_group_hierarchy_lite(
    pool: Arc<ConnectionPool>,
) -> Result<u64, sqlx::Error> {
    log::info!("refreshing materialized view title_group_hierarchy_lite");
    sqlx::query!("REFRESH MATERIALIZED VIEW title_group_hierarchy_lite")
        .execute(<ConnectionPool as Borrow<PgPool>>::borrow(&pool))
        .await?;
    log::info!("materialized view title_group_hierarchy_lite refreshed");
    Ok(0)
}
