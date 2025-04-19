use crate::{Error, Result, models::user::PublicUser};
use sqlx::PgPool;

pub async fn find_user_by_id(pool: &PgPool, id: &i64) -> Result<PublicUser> {
    sqlx::query_as!(
        PublicUser,
        r#"
            SELECT
                id,
                username,
                avatar,
                created_at,
                description,
                uploaded,
                downloaded,
                ratio,
                required_ratio,
                last_seen,
                class,
                forum_posts,
                forum_threads,
                group_comments,
                torrent_comments,
                request_comments,
                artist_comments,
                seeding,
                leeching,
                snatched,
                seeding_size,
                requests_filled,
                collages_started,
                requests_voted,
                average_seeding_time,
                invited,
                invitations,
                bonus_points
            FROM users
            WHERE id = $1
        "#,
        *id as i64
    )
    .fetch_one(pool)
    .await
    .map_err(|_| Error::UserWithIdNotFound(*id))
}

pub async fn update_last_seen(pool: &PgPool, id: i64) -> Result<()> {
    let _ = sqlx::query!(
        r#"
            UPDATE users
            SET last_seen = CURRENT_TIMESTAMP
            WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}
