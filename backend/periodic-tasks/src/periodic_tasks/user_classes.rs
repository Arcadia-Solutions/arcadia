use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::user::UserClass;
use chrono::Utc;
use std::borrow::Borrow;
use std::sync::Arc;

struct UserWithStats {
    id: i32,
    class_name: String,
    class_locked: bool,
    warned: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    uploaded: i64,
    downloaded: i64,
    snatched: i32,
    forum_posts: i32,
    seeding_size: i64,
    torrent_uploads_in_unique_title_groups: i32,
    forum_posts_in_unique_threads: i32,
}

pub async fn process_user_class_changes(pool: Arc<ConnectionPool>) {
    match process_user_class_changes_inner(pool).await {
        Ok((promotions, demotions)) => {
            log::info!(
                "Processed user class changes: {} promotions, {} demotions",
                promotions,
                demotions
            );
        }
        Err(e) => {
            log::error!("Error processing user class changes: {}", e);
        }
    }
}

async fn process_user_class_changes_inner(
    pool: Arc<ConnectionPool>,
) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    // Get all user classes
    let all_classes = pool.get_all_user_classes().await?;

    // Get all users with their stats
    let users = get_all_users_with_stats(&pool).await?;

    let mut promotions = 0;
    let mut demotions = 0;

    for user in users {
        // Skip if class is locked
        if user.class_locked {
            continue;
        }

        // Get current user class
        let current_class = match all_classes.iter().find(|c| c.name == user.class_name) {
            Some(class) => class,
            None => {
                // should never happen, but oh well
                log::warn!("User {} has unknown class '{}'", user.id, user.class_name);
                continue;
            }
        };

        // Check for demotion first
        if current_class.automatic_demotion && !meets_requirements(&user, current_class) {
            // User should be demoted
            if let Some(ref previous_class_name) = current_class.previous_user_class {
                log::info!(
                    "Demoting user {} from {} to {}",
                    user.id,
                    user.class_name,
                    previous_class_name
                );
                match pool
                    .change_user_class(user.id, previous_class_name, true)
                    .await
                {
                    Ok(_) => {
                        demotions += 1;
                    }
                    Err(e) => {
                        log::error!("Error demoting user {}: {}", user.id, e);
                    }
                }
                // Move on to next user after demotion
                continue;
            }
        }

        // Check for promotion (only if not demoted)
        // Find classes where previous_user_class == current user's class
        for next_class in &all_classes {
            if !next_class.automatic_promotion {
                continue;
            }

            // Check if this class references current class as previous
            if next_class.previous_user_class.as_ref() != Some(&user.class_name) {
                continue;
            }

            // Check if user is warned and promotion not allowed while warned
            if user.warned && !next_class.promotion_allowed_while_warned {
                continue;
            }

            // Check if user meets all requirements for promotion
            if meets_requirements(&user, next_class) {
                log::info!(
                    "Promoting user {} from {} to {}",
                    user.id,
                    user.class_name,
                    next_class.name
                );
                match pool
                    .change_user_class(user.id, &next_class.name, true)
                    .await
                {
                    Ok(_) => {
                        promotions += 1;
                        // Only promote one level at a time
                        break;
                    }
                    Err(e) => {
                        log::error!("Error promoting user {}: {}", user.id, e);
                    }
                }
            }
        }
    }

    Ok((promotions, demotions))
}

fn meets_requirements(user: &UserWithStats, class: &UserClass) -> bool {
    // Check account age
    let account_age_days = (Utc::now() - user.created_at).num_days();
    if account_age_days < class.required_account_age_in_days as i64 {
        return false;
    }

    // Check ratio
    if class.required_ratio > 0.0 {
        let ratio = user.uploaded as f64 / user.downloaded as f64;
        if ratio < class.required_ratio {
            return false;
        }
    }

    // Check uploaded
    if user.uploaded < class.required_uploaded {
        return false;
    }

    // Check downloaded
    if user.downloaded < class.required_downloaded {
        return false;
    }

    // Check snatched
    if user.snatched < class.required_torrent_snatched {
        return false;
    }

    // Check forum posts
    if user.forum_posts < class.required_forum_posts {
        return false;
    }

    // Check seeding size
    if user.seeding_size < class.required_seeding_size {
        return false;
    }

    // Check torrent uploads in unique title groups
    if user.torrent_uploads_in_unique_title_groups
        < class.required_torrent_uploads_in_unique_title_groups
    {
        return false;
    }

    // Check forum posts in unique threads
    if user.forum_posts_in_unique_threads < class.required_forum_posts_in_unique_threads {
        return false;
    }

    true
}

async fn get_all_users_with_stats(
    pool: &ConnectionPool,
) -> Result<Vec<UserWithStats>, Box<dyn std::error::Error>> {
    let users = sqlx::query_as!(
        UserWithStats,
        r#"
        SELECT
            u.id,
            u.class_name,
            u.class_locked,
            u.warned,
            u.created_at,
            u.uploaded,
            u.downloaded,
            u.snatched,
            u.forum_posts,
            u.seeding_size,
            COALESCE(
                (SELECT COUNT(DISTINCT eg.title_group_id)
                 FROM torrents t
                 INNER JOIN edition_groups eg ON t.edition_group_id = eg.id
                 WHERE t.created_by_id = u.id),
                0
            )::int as "torrent_uploads_in_unique_title_groups!",
            COALESCE(
                (SELECT COUNT(DISTINCT fp.forum_thread_id)
                 FROM forum_posts fp
                 WHERE fp.created_by_id = u.id),
                0
            )::int as "forum_posts_in_unique_threads!"
        FROM users u
        WHERE u.banned = false
        "#
    )
    .fetch_all(pool.borrow())
    .await?;

    Ok(users)
}
