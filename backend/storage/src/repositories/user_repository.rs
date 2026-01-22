use crate::{
    connection_pool::ConnectionPool,
    models::{
        common::PaginatedResults,
        user::{
            EditedUser, EditedUserClass, PublicUser, SearchUsersQuery, UserClass,
            UserCreatedUserClass, UserCreatedUserWarning, UserLite, UserMinimal, UserPermission,
            UserSearchResult, UserSettings, UserWarning,
        },
    },
};
use arcadia_common::error::{Error, Result};
use arcadia_shared::tracker::models::user::APIUpdateUserMaxSnatchesPerDay;
use reqwest::Client;
use sqlx::PgPool;
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn find_user_profile(&self, id: &i32) -> Result<PublicUser> {
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
                    real_uploaded,
                    real_downloaded,
                    last_seen,
                    class_name,
                    class_locked,
                    title_groups,
                    edition_groups,
                    torrents,
                    forum_posts,
                    forum_threads,
                    title_group_comments,
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
                    bonus_points,
                    warned,
                    banned,
                    custom_title
                FROM users
                WHERE id = $1
            "#,
            *id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::UserWithIdNotFound(*id))
    }

    // this keeps track of when the user made a request for the last time
    // as well as increments the streak if needed
    pub async fn update_last_seen_and_streak(&self, id: i32) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE users
            SET
                highest_streak = GREATEST(
                    highest_streak,
                    CASE
                        WHEN last_seen::date = CURRENT_DATE - INTERVAL '1 day' THEN current_streak + 1
                        WHEN last_seen::date < CURRENT_DATE - INTERVAL '1 day' THEN 1
                        ELSE current_streak
                    END
                ),
                current_streak = CASE
                    WHEN last_seen::date = CURRENT_DATE - INTERVAL '1 day' THEN current_streak + 1
                    WHEN last_seen::date < CURRENT_DATE - INTERVAL '1 day' THEN 1
                    ELSE current_streak
                END,
                last_seen = NOW()
            WHERE id = $1
            "#,
            id
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    pub async fn update_user(&self, user_id: i32, edited_user: &EditedUser) -> Result<()> {
        let _ = sqlx::query!(
            r#"
                UPDATE users
                SET avatar = $2, description = $3, email = $4
                WHERE id = $1
            "#,
            user_id,
            edited_user.avatar,
            edited_user.description,
            edited_user.email
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    pub async fn get_user_settings(&self, user_id: i32) -> Result<UserSettings> {
        let user_settings = sqlx::query_as!(
            UserSettings,
            r#"
                SELECT css_sheet_name
                FROM users
                WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::UserWithIdNotFound(user_id))?;

        Ok(user_settings)
    }

    pub async fn update_user_settings(&self, user_id: i32, settings: &UserSettings) -> Result<()> {
        let _ = sqlx::query!(
            r#"
                UPDATE users
                SET css_sheet_name = $2
                WHERE id = $1
            "#,
            user_id,
            settings.css_sheet_name
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    pub async fn update_user_custom_title(
        &self,
        user_id: i32,
        custom_title: Option<&str>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE users
                SET custom_title = $2
                WHERE id = $1
            "#,
            user_id,
            custom_title
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    pub async fn create_user_warning(
        &self,
        current_user_id: i32,
        user_warning: &UserCreatedUserWarning,
    ) -> Result<UserWarning> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        let _ = sqlx::query!(
            r#"
                UPDATE users
                SET warned = true,
                banned = CASE
                    WHEN $2 IS TRUE THEN TRUE
                    ELSE banned
                END
                WHERE id = $1
            "#,
            user_warning.user_id,
            user_warning.ban
        )
        .execute(&mut *tx)
        .await?;

        let user_warning = sqlx::query_as!(
            UserWarning,
            r#"
                INSERT INTO user_warnings (user_id, expires_at, reason, created_by_id, ban)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING *
            "#,
            user_warning.user_id,
            user_warning.expires_at,
            user_warning.reason,
            current_user_id,
            user_warning.ban
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::CouldNotCreateGift)?;

        tx.commit().await?;

        Ok(user_warning)
    }

    pub async fn find_user_warnings(&self, user_id: i32) -> Vec<UserWarning> {
        sqlx::query_as!(
            UserWarning,
            r#"
                SELECT * FROM user_warnings
                WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(self.borrow())
        .await
        .expect("failed to get user warnings")
    }

    pub async fn is_user_banned(&self, user_id: i32) -> Result<bool> {
        let result = sqlx::query_scalar!("SELECT banned FROM users WHERE id = $1", user_id)
            .fetch_optional(self.borrow())
            .await?;

        let Some(banned) = result else {
            return Ok(true);
        };

        Ok(banned)
    }

    pub async fn find_registered_users(&self) -> Result<Vec<UserMinimal>> {
        let users = sqlx::query_as!(
            UserMinimal,
            r#"
            SELECT id, passkey FROM users
            "#
        )
        .fetch_all(self.borrow())
        .await?;

        Ok(users)
    }

    pub async fn create_user_class(&self, user_class: &UserCreatedUserClass) -> Result<UserClass> {
        sqlx::query_as!(
            UserClass,
            r#"
                INSERT INTO user_classes (
                    name,
                    new_permissions,
                    automatic_promotion,
                    automatic_demotion,
                    promotion_allowed_while_warned,
                    previous_user_class,
                    required_account_age_in_days,
                    required_ratio,
                    required_torrent_uploads,
                    required_torrent_uploads_in_unique_title_groups,
                    required_uploaded,
                    required_torrent_snatched,
                    required_downloaded,
                    required_forum_posts,
                    required_forum_posts_in_unique_threads,
                    required_title_group_comments,
                    required_seeding_size,
                    max_snatches_per_day
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
                RETURNING
                    name,
                    new_permissions as "new_permissions: Vec<UserPermission>",
                    max_snatches_per_day,
                    automatic_promotion,
                    automatic_demotion,
                    promotion_allowed_while_warned,
                    previous_user_class,
                    required_account_age_in_days,
                    required_ratio,
                    required_torrent_uploads,
                    required_torrent_uploads_in_unique_title_groups,
                    required_uploaded,
                    required_torrent_snatched,
                    required_downloaded,
                    required_forum_posts,
                    required_forum_posts_in_unique_threads,
                    required_title_group_comments,
                    required_seeding_size
            "#,
            user_class.name,
            &user_class.new_permissions as &[UserPermission],
            user_class.automatic_promotion,
            user_class.automatic_demotion,
            user_class.promotion_allowed_while_warned,
            user_class.previous_user_class,
            user_class.required_account_age_in_days,
            user_class.required_ratio,
            user_class.required_torrent_uploads,
            user_class.required_torrent_uploads_in_unique_title_groups,
            user_class.required_uploaded,
            user_class.required_torrent_snatched,
            user_class.required_downloaded,
            user_class.required_forum_posts,
            user_class.required_forum_posts_in_unique_threads,
            user_class.required_title_group_comments,
            user_class.required_seeding_size,
            user_class.max_snatches_per_day
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e
                && db_err.code() == Some(std::borrow::Cow::Borrowed("23505"))
            {
                return Error::UserClassAlreadyExists;
            }
            Error::CouldNotCreateUserClass(e)
        })
    }

    pub async fn get_user_class_by_name(&self, name: &str) -> Result<UserClass> {
        sqlx::query_as!(
            UserClass,
            r#"
                SELECT
                    name,
                    new_permissions as "new_permissions: Vec<UserPermission>",
                    max_snatches_per_day,
                    automatic_promotion,
                    automatic_demotion,
                    promotion_allowed_while_warned,
                    previous_user_class,
                    required_account_age_in_days,
                    required_ratio,
                    required_torrent_uploads,
                    required_torrent_uploads_in_unique_title_groups,
                    required_uploaded,
                    required_torrent_snatched,
                    required_downloaded,
                    required_forum_posts,
                    required_forum_posts_in_unique_threads,
                    required_title_group_comments,
                    required_seeding_size
                FROM user_classes
                WHERE name = $1
            "#,
            name
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::UserClassNotFound(name.to_string()))
    }

    pub async fn get_all_user_classes(&self) -> Result<Vec<UserClass>> {
        sqlx::query_as!(
            UserClass,
            r#"
                SELECT
                    name,
                    new_permissions as "new_permissions: Vec<UserPermission>",
                    max_snatches_per_day,
                    automatic_promotion,
                    automatic_demotion,
                    promotion_allowed_while_warned,
                    previous_user_class,
                    required_account_age_in_days,
                    required_ratio,
                    required_torrent_uploads,
                    required_torrent_uploads_in_unique_title_groups,
                    required_uploaded,
                    required_torrent_snatched,
                    required_downloaded,
                    required_forum_posts,
                    required_forum_posts_in_unique_threads,
                    required_title_group_comments,
                    required_seeding_size
                FROM user_classes
                ORDER BY name
            "#
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::from)
    }

    pub async fn update_user_class(
        &self,
        old_name: &str,
        edited_class: &EditedUserClass,
    ) -> Result<UserClass> {
        sqlx::query_as!(
            UserClass,
            r#"
                UPDATE user_classes
                SET
                    name = $2,
                    new_permissions = $3,
                    automatic_promotion = $4,
                    automatic_demotion = $5,
                    promotion_allowed_while_warned = $6,
                    previous_user_class = $7,
                    required_account_age_in_days = $8,
                    required_ratio = $9,
                    required_torrent_uploads = $10,
                    required_torrent_uploads_in_unique_title_groups = $11,
                    required_uploaded = $12,
                    required_torrent_snatched = $13,
                    required_downloaded = $14,
                    required_forum_posts = $15,
                    required_forum_posts_in_unique_threads = $16,
                    required_title_group_comments = $17,
                    required_seeding_size = $18,
                    max_snatches_per_day = $19
                WHERE name = $1
                RETURNING
                    name,
                    new_permissions as "new_permissions: Vec<UserPermission>",
                    automatic_promotion,
                    automatic_demotion,
                    promotion_allowed_while_warned,
                    previous_user_class,
                    required_account_age_in_days,
                    required_ratio,
                    required_torrent_uploads,
                    required_torrent_uploads_in_unique_title_groups,
                    required_uploaded,
                    required_torrent_snatched,
                    required_downloaded,
                    required_forum_posts,
                    required_forum_posts_in_unique_threads,
                    required_title_group_comments,
                    required_seeding_size,
                    max_snatches_per_day
            "#,
            old_name,
            edited_class.name,
            &edited_class.new_permissions as &[UserPermission],
            edited_class.automatic_promotion,
            edited_class.automatic_demotion,
            edited_class.promotion_allowed_while_warned,
            edited_class.previous_user_class,
            edited_class.required_account_age_in_days,
            edited_class.required_ratio,
            edited_class.required_torrent_uploads,
            edited_class.required_torrent_uploads_in_unique_title_groups,
            edited_class.required_uploaded,
            edited_class.required_torrent_snatched,
            edited_class.required_downloaded,
            edited_class.required_forum_posts,
            edited_class.required_forum_posts_in_unique_threads,
            edited_class.required_title_group_comments,
            edited_class.required_seeding_size,
            edited_class.max_snatches_per_day
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|e| {
            if let sqlx::Error::RowNotFound = e {
                Error::UserClassNotFound(old_name.to_string())
            } else if let sqlx::Error::Database(ref db_err) = e {
                if db_err.code() == Some(std::borrow::Cow::Borrowed("23505")) {
                    return Error::UserClassAlreadyExists;
                }
                Error::CouldNotUpdateUserClass(e)
            } else {
                Error::CouldNotUpdateUserClass(e)
            }
        })
    }

    pub async fn delete_user_class(&self, name: &str, target_class_name: &str) -> Result<()> {
        // Verify target class exists
        self.get_user_class_by_name(target_class_name).await?;

        // Migrate all users from the deleted class to the target class
        sqlx::query!(
            r#"
                UPDATE users
                SET class_name = $2
                WHERE class_name = $1
            "#,
            name,
            target_class_name
        )
        .execute(self.borrow())
        .await?;

        // Delete the user class
        let result = sqlx::query!(r#"DELETE FROM user_classes WHERE name = $1"#, name)
            .execute(self.borrow())
            .await
            .map_err(Error::CouldNotDeleteUserClass)?;

        if result.rows_affected() == 0 {
            return Err(Error::UserClassNotFound(name.to_string()));
        }

        Ok(())
    }

    pub async fn count_users_in_class(&self, class_name: &str) -> Result<i64> {
        let result = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM users WHERE class_name = $1"#,
            class_name
        )
        .fetch_one(self.borrow())
        .await?;

        Ok(result.unwrap_or(0))
    }

    pub async fn update_user_permissions(
        &self,
        user_id: i32,
        permissions: &[UserPermission],
    ) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE users
                SET permissions = $2
                WHERE id = $1
            "#,
            user_id,
            permissions as &[UserPermission]
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    pub async fn lock_user_class(&self, user_id: i32, locked: bool) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE users
                SET class_locked = $2
                WHERE id = $1
            "#,
            user_id,
            locked
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    /// Changes the user's class and updates their permissions accordingly.
    /// Set `notify_tracker` to false in tests to skip the HTTP call to the tracker.
    pub async fn change_user_class(
        &self,
        user_id: i32,
        new_class_name: &str,
        notify_tracker: bool,
    ) -> Result<()> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        // Get current user with their class and permissions
        let user = sqlx::query_as!(
            crate::models::user::User,
            r#"
                SELECT id, username, avatar, email, password_hash, registered_from_ip, created_at,
                       description, uploaded, real_uploaded, downloaded, real_downloaded, last_seen,
                       class_name, class_locked, permissions as "permissions: Vec<UserPermission>",
                       title_groups, edition_groups, torrents, forum_posts, forum_threads,
                       title_group_comments, request_comments, artist_comments, seeding, leeching,
                       snatched, seeding_size, requests_filled, collages_started, requests_voted,
                       average_seeding_time, invited, invitations, bonus_points, freeleech_tokens,
                       warned, banned, staff_note, passkey, css_sheet_name, current_streak,
                       highest_streak, custom_title, max_snatches_per_day
                FROM users
                WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| Error::UserWithIdNotFound(user_id))?;

        let old_class_name = user.class_name.clone();

        // If the class hasn't changed, do nothing
        if old_class_name == new_class_name {
            return Ok(());
        }

        // Get old class details
        let old_class = sqlx::query_as!(
            crate::models::user::UserClass,
            r#"
                SELECT
                    name,
                    new_permissions as "new_permissions: Vec<UserPermission>",
                    max_snatches_per_day,
                    automatic_promotion,
                    automatic_demotion,
                    promotion_allowed_while_warned,
                    previous_user_class,
                    required_account_age_in_days,
                    required_ratio,
                    required_torrent_uploads,
                    required_torrent_uploads_in_unique_title_groups,
                    required_uploaded,
                    required_torrent_snatched,
                    required_downloaded,
                    required_forum_posts,
                    required_forum_posts_in_unique_threads,
                    required_title_group_comments,
                    required_seeding_size
                FROM user_classes
                WHERE name = $1
            "#,
            old_class_name
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| Error::UserClassNotFound(old_class_name.clone()))?;

        // Get new class details
        let new_class = sqlx::query_as!(
            crate::models::user::UserClass,
            r#"
                SELECT
                    name,
                    new_permissions as "new_permissions: Vec<UserPermission>",
                    automatic_promotion,
                    automatic_demotion,
                    promotion_allowed_while_warned,
                    previous_user_class,
                    max_snatches_per_day,
                    required_account_age_in_days,
                    required_ratio,
                    required_torrent_uploads,
                    required_torrent_uploads_in_unique_title_groups,
                    required_uploaded,
                    required_torrent_snatched,
                    required_downloaded,
                    required_forum_posts,
                    required_forum_posts_in_unique_threads,
                    required_title_group_comments,
                    required_seeding_size
                FROM user_classes
                WHERE name = $1
            "#,
            new_class_name
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|_| Error::UserClassNotFound(new_class_name.to_string()))?;

        let mut updated_permissions = user.permissions.clone();

        // Determine if it's a 1-hop promotion or demotion
        let is_promotion = new_class.previous_user_class.as_ref() == Some(&old_class_name);
        let is_demotion =
            old_class.previous_user_class.as_ref() == Some(&new_class_name.to_string());

        if is_promotion {
            // Add new class permissions (avoiding duplicates)
            for perm in &new_class.new_permissions {
                if !updated_permissions.contains(perm) {
                    updated_permissions.push(perm.clone());
                }
            }
        } else if is_demotion {
            // Remove old class permissions
            updated_permissions.retain(|p| !old_class.new_permissions.contains(p));
        }

        // Deduplicate permissions to ensure no duplicates exist
        let mut deduped = Vec::new();
        for perm in updated_permissions {
            if !deduped.contains(&perm) {
                deduped.push(perm);
            }
        }
        let updated_permissions = deduped;

        // Update user's class and permissions
        sqlx::query!(
            r#"
                UPDATE users
                SET class_name = $2, permissions = $3, max_snatches_per_day = $4
                WHERE id = $1
            "#,
            user_id,
            new_class_name,
            &updated_permissions as &[UserPermission],
            new_class.max_snatches_per_day
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        // Notify tracker about the new max_snatches_per_day
        if notify_tracker {
            let tracker_config = &self.tracker_config;
            let mut url = tracker_config.url_internal.clone();
            url.path_segments_mut()
                .unwrap()
                .push("api")
                .push("users")
                .push(&user_id.to_string())
                .push("max-snatches-per-day");

            let payload = APIUpdateUserMaxSnatchesPerDay {
                id: user_id as u32,
                max_snatches_per_day: new_class.max_snatches_per_day.map(|x| x as u32),
            };

            if let Err(e) = Client::new()
                .put(url)
                .header("x-api-key", tracker_config.api_key.clone())
                .json(&payload)
                .send()
                .await
            {
                log::warn!("Failed to update user snatch limit in tracker: {}", e);
            }
        }

        Ok(())
    }

    pub async fn find_users_lite(&self, username: &String) -> Result<Vec<UserLite>> {
        let found_users = sqlx::query_as!(
            UserLite,
            r#"
            SELECT id, username, warned, banned
            FROM users
            WHERE LOWER(username) LIKE LOWER('%' || $1 || '%')
            LIMIT 5
            "#,
            username
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotSearchForUsers)?;

        Ok(found_users)
    }

    pub async fn get_user_torrent_clients(
        &self,
        user_id: i32,
    ) -> Result<Vec<crate::models::peer::TorrentClient>> {
        let clients = sqlx::query_as!(
            crate::models::peer::TorrentClient,
            r#"
            SELECT
                ip,
                port,
                MIN(created_at)::timestamptz AS "first_seen_at!",
                MAX(updated_at)::timestamptz AS "last_seen_at!",
                SUM(uploaded)::bigint AS "real_uploaded!",
                SUM(downloaded)::bigint AS "real_downloaded!",
                agent
            FROM peers
            WHERE user_id = $1
            GROUP BY agent, ip, port
            ORDER BY agent, ip, port
            "#,
            user_id
        )
        .fetch_all(self.borrow())
        .await?;

        Ok(clients)
    }

    pub async fn search_users(
        &self,
        query: &SearchUsersQuery,
    ) -> Result<PaginatedResults<UserSearchResult>> {
        let limit = query.page_size as i64;
        let offset = (query.page - 1) as i64 * query.page_size as i64;
        let order_by = query.order_by.to_string();
        let is_asc = matches!(
            query.order_by_direction,
            crate::models::common::OrderByDirection::Asc
        );

        let results = sqlx::query_as!(
            UserSearchResult,
            r#"
            SELECT id, username, avatar, class_name, created_at, last_seen, uploaded, downloaded,
                   torrents, title_groups, title_group_comments, forum_posts, forum_threads, warned, banned
            FROM users
            WHERE ($1::TEXT IS NULL OR LOWER(username) LIKE LOWER('%' || $1 || '%'))
            ORDER BY
                CASE WHEN $2 = 'username' AND $3 THEN username END ASC,
                CASE WHEN $2 = 'username' AND NOT $3 THEN username END DESC,
                CASE WHEN $2 = 'created_at' AND $3 THEN created_at END ASC,
                CASE WHEN $2 = 'created_at' AND NOT $3 THEN created_at END DESC,
                CASE WHEN $2 = 'last_seen' AND $3 THEN last_seen END ASC,
                CASE WHEN $2 = 'last_seen' AND NOT $3 THEN last_seen END DESC,
                CASE WHEN $2 = 'uploaded' AND $3 THEN uploaded END ASC,
                CASE WHEN $2 = 'uploaded' AND NOT $3 THEN uploaded END DESC,
                CASE WHEN $2 = 'downloaded' AND $3 THEN downloaded END ASC,
                CASE WHEN $2 = 'downloaded' AND NOT $3 THEN downloaded END DESC,
                CASE WHEN $2 = 'torrents' AND $3 THEN torrents END ASC,
                CASE WHEN $2 = 'torrents' AND NOT $3 THEN torrents END DESC,
                CASE WHEN $2 = 'title_groups' AND $3 THEN title_groups END ASC,
                CASE WHEN $2 = 'title_groups' AND NOT $3 THEN title_groups END DESC,
                CASE WHEN $2 = 'title_group_comments' AND $3 THEN title_group_comments END ASC,
                CASE WHEN $2 = 'title_group_comments' AND NOT $3 THEN title_group_comments END DESC,
                CASE WHEN $2 = 'forum_posts' AND $3 THEN forum_posts END ASC,
                CASE WHEN $2 = 'forum_posts' AND NOT $3 THEN forum_posts END DESC,
                CASE WHEN $2 = 'forum_threads' AND $3 THEN forum_threads END ASC,
                CASE WHEN $2 = 'forum_threads' AND NOT $3 THEN forum_threads END DESC
            LIMIT $4 OFFSET $5
            "#,
            query.username,
            order_by,
            is_asc,
            limit,
            offset
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotSearchForUsers)?;

        let total_items = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM users
            WHERE ($1::TEXT IS NULL OR LOWER(username) LIKE LOWER('%' || $1 || '%'))
            "#,
            query.username
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotSearchForUsers)?;

        Ok(PaginatedResults {
            results,
            page: query.page,
            page_size: query.page_size,
            total_items,
        })
    }
}
