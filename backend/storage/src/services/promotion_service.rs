use crate::models::user::{UserClass, UserWithStats};
use chrono::Utc;

/// Check if a user meets all the requirements for a specific user class.
pub fn meets_requirements(user: &UserWithStats, class: &UserClass) -> bool {
    let account_age_days = (Utc::now() - user.created_at).num_days();
    if account_age_days < class.required_account_age_in_days as i64 {
        return false;
    }

    if class.required_ratio > 0.0 {
        let ratio = user.uploaded as f64 / user.downloaded as f64;
        if ratio < class.required_ratio {
            return false;
        }
    }

    if user.uploaded < class.required_uploaded {
        return false;
    }

    if user.downloaded < class.required_downloaded {
        return false;
    }

    if user.snatched < class.required_torrent_snatched {
        return false;
    }

    if user.forum_posts < class.required_forum_posts {
        return false;
    }

    if user.seeding_size < class.required_seeding_size {
        return false;
    }

    if user.torrent_uploads_in_unique_title_groups
        < class.required_torrent_uploads_in_unique_title_groups
    {
        return false;
    }

    if user.forum_posts_in_unique_threads < class.required_forum_posts_in_unique_threads {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::UserPermission;
    use chrono::TimeZone;

    fn create_test_user() -> UserWithStats {
        UserWithStats {
            id: 1,
            class_name: "user".to_string(),
            class_locked: false,
            warned: false,
            bonus_points: 1000,
            created_at: Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
            uploaded: 10_000_000_000,  // 10 GB
            downloaded: 5_000_000_000, // 5 GB
            snatched: 50,
            forum_posts: 100,
            seeding_size: 20_000_000_000, // 20 GB
            torrent_uploads_in_unique_title_groups: 10,
            forum_posts_in_unique_threads: 20,
        }
    }

    fn create_test_class() -> UserClass {
        UserClass {
            name: "power_user".to_string(),
            new_permissions: vec![UserPermission::UploadTorrent],
            max_snatches_per_day: None,
            automatic_promotion: true,
            automatic_demotion: true,
            promotion_allowed_while_warned: false,
            previous_user_class: Some("user".to_string()),
            required_account_age_in_days: 30,
            required_ratio: 1.0,
            required_torrent_uploads: 5,
            required_torrent_uploads_in_unique_title_groups: 5,
            required_uploaded: 5_000_000_000,
            required_torrent_snatched: 10,
            required_downloaded: 0,
            required_forum_posts: 10,
            required_forum_posts_in_unique_threads: 5,
            required_title_group_comments: 0,
            required_seeding_size: 10_000_000_000,
            promotion_cost_bonus_points: 500,
        }
    }

    #[test]
    fn test_meets_requirements_success() {
        let user = create_test_user();
        let class = create_test_class();
        assert!(meets_requirements(&user, &class));
    }

    #[test]
    fn test_meets_requirements_fails_on_account_age() {
        let mut user = create_test_user();
        user.created_at = Utc::now(); // Just created
        let class = create_test_class();
        assert!(!meets_requirements(&user, &class));
    }

    #[test]
    fn test_meets_requirements_fails_on_ratio() {
        let mut user = create_test_user();
        user.uploaded = 1_000_000_000; // 1 GB
        user.downloaded = 10_000_000_000; // 10 GB (ratio = 0.1)
        let class = create_test_class();
        assert!(!meets_requirements(&user, &class));
    }

    #[test]
    fn test_meets_requirements_fails_on_uploaded() {
        let mut user = create_test_user();
        user.uploaded = 1_000_000_000; // 1 GB (less than required 5 GB)
        let class = create_test_class();
        assert!(!meets_requirements(&user, &class));
    }
}
