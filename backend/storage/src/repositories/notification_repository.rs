use crate::{
    connection_pool::ConnectionPool,
    models::notification::{
        NotificationForumThreadPost, NotificationStaffPmMessage, NotificationTitleGroupComment,
    },
};
use arcadia_common::error::{Error, Result};
use sqlx::{Postgres, Transaction};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn notify_users_title_group_torrents(
        tx: &mut Transaction<'_, Postgres>,
        title_group_id: i32,
        torrent_id: i32,
        current_user_id: i32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                WITH user_ids AS (
                    SELECT user_id
                    FROM subscriptions_title_group_torrents
                    WHERE title_group_id = $1
                    AND user_id != $3
                )
                INSERT INTO notifications_title_group_torrents (user_id, torrent_id)
                SELECT
                    user_id,
                    $2
                FROM user_ids
            "#,
            title_group_id,
            torrent_id,
            current_user_id
        )
        .execute(&mut **tx)
        .await
        .map_err(Error::CouldNotCreateNotification)?;

        Ok(())
    }

    pub async fn notify_users_forum_thread_posts(
        tx: &mut Transaction<'_, Postgres>,
        thread_id: i64,
        post_id: i64,
        current_user_id: i32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                WITH user_ids AS (
                    SELECT user_id
                    FROM subscriptions_forum_thread_posts
                    WHERE forum_thread_id = $1
                    AND user_id != $3
                )
                INSERT INTO notifications_forum_thread_posts (user_id, forum_post_id, forum_thread_id)
                SELECT
                    user_id,
                    $2,
                    $1
                FROM user_ids u
                -- don't notify the user who created the post
                WHERE NOT EXISTS (
                    SELECT 1
                    FROM notifications_forum_thread_posts n
                    WHERE n.user_id = u.user_id
                      AND n.forum_thread_id = $1
                      AND n.read_status = FALSE
                )
            "#,
            thread_id,
            post_id,
            current_user_id
        )
        .execute(&mut **tx)
        .await
        .map_err(Error::CouldNotCreateNotification)?;

        Ok(())
    }

    pub async fn find_unread_notifications_amount_forum_thread_posts(
        &self,
        user_id: i32,
    ) -> Result<i64> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM notifications_forum_thread_posts
            WHERE user_id = $1 AND read_status = FALSE
            "#,
            user_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotGetUnreadNotifications)?
        .unwrap_or(0);

        Ok(count)
    }

    pub async fn find_unread_notifications_amount_title_group_torrents(
        &self,
        user_id: i32,
    ) -> Result<i64> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM notifications_title_group_torrents
            WHERE user_id = $1 AND read_status = FALSE
            "#,
            user_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotGetUnreadNotifications)?
        .unwrap_or(0);

        Ok(count)
    }

    pub async fn find_notifications_forum_thread_posts(
        &self,
        user_id: i32,
        include_read: bool,
    ) -> Result<Vec<NotificationForumThreadPost>> {
        let notifications = sqlx::query_as!(
            NotificationForumThreadPost,
            r#"
            SELECT
                n.id,
                n.forum_post_id,
                p.forum_thread_id,
                t.name AS forum_thread_name,
                n.created_at,
                n.read_status
            FROM notifications_forum_thread_posts n
            JOIN forum_posts p ON p.id = n.forum_post_id
            JOIN forum_threads t ON t.id = n.forum_thread_id
            WHERE n.user_id = $1
            AND ($2::bool = TRUE OR n.read_status = FALSE)
            ORDER BY n.created_at DESC
            "#,
            user_id,
            include_read
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotGetUnreadNotifications)?;

        Ok(notifications)
    }

    pub async fn mark_notification_forum_thread_post_as_read(
        &self,
        forum_thread_id: i64,
        user_id: i32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE notifications_forum_thread_posts
                SET read_status = TRUE
                WHERE forum_thread_id = $1 AND user_id = $2
            "#,
            forum_thread_id,
            user_id
        )
        .execute(self.borrow())
        .await
        .map_err(Error::CouldNotFindForumThread)?;

        Ok(())
    }

    pub async fn notify_users_title_group_comments(
        tx: &mut Transaction<'_, Postgres>,
        title_group_id: i32,
        comment_id: i64,
        current_user_id: i32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                WITH user_ids AS (
                    SELECT user_id
                    FROM subscriptions_title_group_comments
                    WHERE title_group_id = $1
                    AND user_id != $3
                )
                INSERT INTO notifications_title_group_comments (user_id, title_group_comment_id, title_group_id)
                SELECT
                    user_id,
                    $2,
                    $1
                FROM user_ids u
                WHERE NOT EXISTS (
                    SELECT 1
                    FROM notifications_title_group_comments n
                    WHERE n.user_id = u.user_id
                      AND n.title_group_id = $1
                      AND n.read_status = FALSE
                )
            "#,
            title_group_id,
            comment_id,
            current_user_id
        )
        .execute(&mut **tx)
        .await
        .map_err(Error::CouldNotCreateNotification)?;

        Ok(())
    }

    pub async fn find_unread_notifications_amount_title_group_comments(
        &self,
        user_id: i32,
    ) -> Result<i64> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM notifications_title_group_comments
            WHERE user_id = $1 AND read_status = FALSE
            "#,
            user_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotGetUnreadNotifications)?
        .unwrap_or(0);

        Ok(count)
    }

    pub async fn find_notifications_title_group_comments(
        &self,
        user_id: i32,
        include_read: bool,
    ) -> Result<Vec<NotificationTitleGroupComment>> {
        let notifications = sqlx::query_as!(
            NotificationTitleGroupComment,
            r#"
            SELECT
                n.id,
                n.title_group_comment_id,
                n.title_group_id,
                tg.name AS title_group_name,
                n.created_at,
                n.read_status
            FROM notifications_title_group_comments n
            JOIN title_groups tg ON tg.id = n.title_group_id
            WHERE n.user_id = $1
            AND ($2::bool = TRUE OR n.read_status = FALSE)
            ORDER BY n.created_at DESC
            "#,
            user_id,
            include_read
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotGetUnreadNotifications)?;

        Ok(notifications)
    }

    pub async fn mark_notification_title_group_comment_as_read(
        &self,
        title_group_id: i32,
        user_id: i32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE notifications_title_group_comments
                SET read_status = TRUE
                WHERE title_group_id = $1 AND user_id = $2
            "#,
            title_group_id,
            user_id
        )
        .execute(self.borrow())
        .await
        .map_err(Error::CouldNotMarkNotificationAsRead)?;

        Ok(())
    }

    pub async fn notify_users_staff_pm_messages(
        tx: &mut Transaction<'_, Postgres>,
        staff_pm_id: i64,
        staff_pm_message_id: i64,
        current_user_id: i32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                WITH eligible_users AS (
                    -- Staff PM creator
                    SELECT created_by_id AS user_id
                    FROM staff_pms
                    WHERE id = $1
                    UNION
                    -- Users with read_staff_pm permission
                    SELECT id AS user_id
                    FROM users
                    WHERE 'read_staff_pm' = ANY(permissions)
                )
                INSERT INTO notifications_staff_pm_messages (user_id, staff_pm_id, staff_pm_message_id)
                SELECT
                    user_id,
                    $1,
                    $2
                FROM eligible_users u
                WHERE u.user_id != $3
                AND NOT EXISTS (
                    SELECT 1
                    FROM notifications_staff_pm_messages n
                    WHERE n.user_id = u.user_id
                      AND n.staff_pm_id = $1
                      AND n.read_status = FALSE
                )
            "#,
            staff_pm_id,
            staff_pm_message_id,
            current_user_id
        )
        .execute(&mut **tx)
        .await
        .map_err(Error::CouldNotCreateNotification)?;

        Ok(())
    }

    pub async fn find_unread_notifications_amount_staff_pm_messages(
        &self,
        user_id: i32,
    ) -> Result<i64> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM notifications_staff_pm_messages
            WHERE user_id = $1 AND read_status = FALSE
            "#,
            user_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotGetUnreadNotifications)?
        .unwrap_or(0);

        Ok(count)
    }

    pub async fn find_notifications_staff_pm_messages(
        &self,
        user_id: i32,
        include_read: bool,
    ) -> Result<Vec<NotificationStaffPmMessage>> {
        let notifications = sqlx::query_as!(
            NotificationStaffPmMessage,
            r#"
            SELECT
                n.id,
                n.staff_pm_message_id,
                n.staff_pm_id,
                sp.subject AS staff_pm_subject,
                n.created_at,
                n.read_status
            FROM notifications_staff_pm_messages n
            JOIN staff_pms sp ON sp.id = n.staff_pm_id
            WHERE n.user_id = $1
            AND ($2::bool = TRUE OR n.read_status = FALSE)
            ORDER BY n.created_at DESC
            "#,
            user_id,
            include_read
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotGetUnreadNotifications)?;

        Ok(notifications)
    }

    pub async fn mark_notifications_staff_pm_messages_as_read(
        &self,
        staff_pm_id: i64,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE notifications_staff_pm_messages
                SET read_status = TRUE
                WHERE staff_pm_id = $1
            "#,
            staff_pm_id
        )
        .execute(self.borrow())
        .await
        .map_err(Error::CouldNotMarkNotificationAsRead)?;

        Ok(())
    }

    pub async fn mark_notification_staff_pm_message_as_read(
        &self,
        staff_pm_id: i64,
        user_id: i32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE notifications_staff_pm_messages
                SET read_status = TRUE
                WHERE staff_pm_id = $1 AND user_id = $2
            "#,
            staff_pm_id,
            user_id
        )
        .execute(self.borrow())
        .await
        .map_err(Error::CouldNotMarkNotificationAsRead)?;

        Ok(())
    }
}
