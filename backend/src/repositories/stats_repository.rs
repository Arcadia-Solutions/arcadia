use sqlx::PgPool;

use crate::{Result, handlers::home_handler::HomeStats};

pub async fn find_home_stats(pool: &PgPool) -> Result<HomeStats> {
    let stats = sqlx::query_as!(
        HomeStats,
        r#"
        SELECT
            (SELECT COUNT(*) FROM users)::BIGINT AS "enabled_users!",
            (SELECT COUNT(*) FROM users WHERE last_seen >= NOW() - INTERVAL '1 day')::BIGINT AS "users_active_today!",
            (SELECT COUNT(*) FROM users WHERE last_seen >= date_trunc('week', NOW()))::BIGINT AS "users_active_this_week!",
            (SELECT COUNT(*) FROM users WHERE last_seen >= date_trunc('month', NOW()))::BIGINT AS "users_active_this_month!",
            (SELECT COUNT(*) FROM title_groups)::BIGINT AS "titles!",
            (SELECT COUNT(*) FROM torrents)::BIGINT AS "torrents!",
            (SELECT COUNT(*) FROM torrents WHERE created_at >= NOW() - INTERVAL '1 day')::BIGINT AS "torrents_uploaded_today!",
            (SELECT COUNT(*) FROM artists)::BIGINT AS "artists!",
            (SELECT COUNT(*) FROM entities)::BIGINT AS "entities!"
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(stats)
}
