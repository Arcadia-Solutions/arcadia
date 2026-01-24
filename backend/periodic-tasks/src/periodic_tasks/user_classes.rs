use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::services::promotion_service::meets_requirements;
use std::sync::Arc;

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
    let users = pool.get_all_users_with_stats().await?;

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
