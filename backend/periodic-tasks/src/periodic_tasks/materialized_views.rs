use arcadia_storage::connection_pool::ConnectionPool;
use std::borrow::Borrow;
use std::sync::Arc;

pub async fn refresh_title_group_hierarchy_lite(pool: Arc<ConnectionPool>) {
    log::info!("refreshing materialized view title_group_hierarchy_lite");
    match refresh_title_group_hierarchy_lite_inner(&pool).await {
        Ok(()) => {
            log::info!("materialized view title_group_hierarchy_lite refreshed");
        }
        Err(e) => {
            log::error!(
                "error refreshing materialized view title_group_hierarchy_lite: {}",
                e
            );
        }
    }
}

async fn refresh_title_group_hierarchy_lite_inner(
    pool: &ConnectionPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!("REFRESH MATERIALIZED VIEW title_group_hierarchy_lite")
        .execute(pool.borrow())
        .await?;

    Ok(())
}
