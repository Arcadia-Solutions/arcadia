use crate::{Error, Result, models::notification::NotificationReason};
use sqlx::PgPool;
use sqlx::{Postgres, Transaction};
use std::collections::HashMap;

pub struct NotificationItemsIds {
    pub title_group_id: Option<i64>,
    pub torrent_id: Option<i64>,
    #[allow(dead_code)]
    pub artist_id: Option<i64>,
    #[allow(dead_code)]
    pub collage_id: Option<i64>,
    #[allow(dead_code)]
    pub forum_thread_id: Option<i64>,
}

pub async fn notify_users(
    tx: &mut Transaction<'_, Postgres>,
    reason: &NotificationReason,
    message: Option<&String>,
    notification_items_ids: NotificationItemsIds,
) -> Result<()> {
    match reason {
        NotificationReason::TorrentUploadedInSubscribedTitleGroup => {
            sqlx::query!(
                r#"
                    WITH subscribers_ids AS (
                        SELECT subscriber_id AS user_id
                        FROM subscriptions
                        WHERE title_group_id = $1
                    )
                    INSERT INTO notifications (receiver_id, reason, torrent_id, title_group_id)
                    SELECT
                        user_id,
                        'TorrentUploadedInSubscribedTitleGroup'::notification_reason_enum,
                        $2,
                        $1
                    FROM subscribers_ids
                "#,
                notification_items_ids.title_group_id,
                notification_items_ids.torrent_id
            )
            .execute(&mut **tx)
            .await
            .map_err(Error::CouldNotCreateNotification)?;
        }
        NotificationReason::SeedingTorrentDeleted => {
            sqlx::query!(
                r#"
                    WITH seeders_ids AS (
                        SELECT user_id
                        FROM torrent_activities
                        WHERE torrent_id = $1
                    )
                    INSERT INTO notifications (receiver_id, reason, message, title_group_id)
                    SELECT
                        user_id,
                        'SeedingTorrentDeleted'::notification_reason_enum,
                        $2,
                        $3
                    FROM seeders_ids
                "#,
                notification_items_ids.torrent_id,
                message,
                notification_items_ids.title_group_id
            )
            .execute(&mut **tx)
            .await
            .map_err(Error::CouldNotCreateNotification)?;
        }
        _ => {
            return Err(Error::UnsupportedNotification);
        }
    }

    Ok(())
}

pub async fn find_unread_notifications_amount(
    pool: &PgPool,
    user_id: i64,
) -> Result<HashMap<NotificationReason, i64>> {
    let rows = sqlx::query!(
        r#"
        SELECT reason as "reason: NotificationReason", 
               COUNT(*) as "count!"
        FROM notifications
        WHERE receiver_id = $1 AND read_status = FALSE
        GROUP BY reason
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(Error::CouldNotGetUnreadNotifications)?;

    let map = rows
        .into_iter()
        .map(|r| (r.reason, r.count))
        .collect::<HashMap<_, _>>();

    Ok(map)
}


#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Executor, PgPool, postgres::PgPoolOptions};
    use std::env;

   #[tokio::test]
async fn test_find_unread_notifications_amount() {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set for tests");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("failed to connect to Postgres");

    let user_id: i64 = 4;

   
    // Clear out old notifications
    pool.execute(sqlx::query!(
        r#"DELETE FROM notifications WHERE receiver_id = $1"#,
        user_id
    ))
    .await
    .unwrap();

    // Insert test notifications
    pool.execute(sqlx::query!(
        r#"
        INSERT INTO notifications (receiver_id, reason, read_status)
        VALUES ($1, 'SeedingTorrentDeleted'::notification_reason_enum, FALSE),
               ($1, 'SeedingTorrentDeleted'::notification_reason_enum, FALSE),
               ($1, 'TorrentUploadedInSubscribedTitleGroup'::notification_reason_enum, FALSE),
               ($1, 'TorrentUploadedInSubscribedTitleGroup'::notification_reason_enum, TRUE) -- ignored
        "#,
        user_id
    ))
    .await
    .unwrap();

    let result = find_unread_notifications_amount(&pool, user_id)
        .await
        .unwrap();

    // Assertions
    assert_eq!(result.get(&NotificationReason::SeedingTorrentDeleted), Some(&2));
    assert_eq!(result.get(&NotificationReason::TorrentUploadedInSubscribedTitleGroup), Some(&1));
    assert!(result.get(&NotificationReason::TorrentUploadedInSubscribedTitleGroup).is_some());
    assert_ne!(result.get(&NotificationReason::TorrentUploadedInSubscribedTitleGroup), Some(&2));
}

}
