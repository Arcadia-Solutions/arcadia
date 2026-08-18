use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::user::UserClass;
use arcadia_storage::services::promotion_service::meets_requirements;
use std::collections::HashMap;
use std::sync::Arc;

pub async fn process_user_class_changes(
    pool: Arc<ConnectionPool>,
) -> Result<u64, Box<dyn std::error::Error>> {
    const BATCH_SIZE: i64 = 100;

    // Get all user classes
    let all_classes = pool.get_all_user_classes().await?;

    let classes_by_name: HashMap<&str, &UserClass> = all_classes
        .iter()
        .map(|class| (class.name.as_str(), class))
        .collect();

    // Classes a user can be automatically promoted to, indexed by the class the user is currently in
    let mut promotion_targets_by_previous_class: HashMap<&str, Vec<&UserClass>> = HashMap::new();
    for class in &all_classes {
        if !class.automatic_promotion {
            continue;
        }
        if let Some(previous_class_name) = &class.previous_user_class {
            promotion_targets_by_previous_class
                .entry(previous_class_name.as_str())
                .or_default()
                .push(class);
        }
    }

    // Only the users in a class that can be automatically left are worth fetching and evaluating
    let class_names_with_possible_change: Vec<String> = all_classes
        .iter()
        .filter(|class| {
            (class.automatic_demotion && class.previous_user_class.is_some())
                || promotion_targets_by_previous_class.contains_key(class.name.as_str())
        })
        .map(|class| class.name.clone())
        .collect();

    if class_names_with_possible_change.is_empty() {
        log::info!("Processed user class changes: 0 promotions, 0 demotions");
        return Ok(0);
    }

    let mut promotions: u64 = 0;
    let mut demotions: u64 = 0;
    let mut last_user_id: i32 = 0;

    loop {
        let users = pool
            .get_users_with_stats(BATCH_SIZE, last_user_id, &class_names_with_possible_change)
            .await?;
        let batch_length = users.len() as i64;

        if let Some(last_user) = users.last() {
            last_user_id = last_user.id;
        }

        for user in users {
            // Get current user class
            let current_class = match classes_by_name.get(user.class_name.as_str()) {
                Some(class) => *class,
                None => {
                    // should never happen, but oh well
                    log::warn!("User {} has unknown class '{}'", user.id, user.class_name);
                    continue;
                }
            };

            // Check for demotion first
            if current_class.automatic_demotion
                && !meets_requirements(&user, current_class)
                && let Some(ref previous_class_name) = current_class.previous_user_class
            {
                // User should be demoted
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

            // Check for promotion (only if not demoted)
            let Some(next_classes) =
                promotion_targets_by_previous_class.get(user.class_name.as_str())
            else {
                continue;
            };

            for next_class in next_classes {
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

        if batch_length < BATCH_SIZE {
            break;
        }
    }

    log::info!(
        "Processed user class changes: {} promotions, {} demotions",
        promotions,
        demotions
    );
    Ok(promotions + demotions)
}
