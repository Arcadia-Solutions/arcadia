use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

/// How long the author of a note may still edit or remove it, without holding the
/// `edit_user_staff_notes` permission.
pub const STAFF_NOTE_EDIT_WINDOW_IN_HOURS: i64 = 24;

/// A note a staff member left on the profile of a user, along with the member of the staff who
/// wrote it.
#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct UserStaffNoteWithAuthor {
    pub id: i64,
    pub user_id: i32,
    #[schema(value_type = String, format = DateTime)]
    pub created_at: DateTime<Utc>,
    pub content: String,
    pub author_id: i32,
    pub author_username: String,
}

impl UserStaffNoteWithAuthor {
    /// Whether a user may edit or remove a note. A user with the `edit_user_staff_notes`
    /// permission may act on every note, its author may act on it during the
    /// [`STAFF_NOTE_EDIT_WINDOW_IN_HOURS`] hours following its creation.
    pub fn can_be_edited_by(&self, user_id: i32, has_permission: bool) -> bool {
        has_permission
            || (self.author_id == user_id
                && (Utc::now() - self.created_at).num_hours() < STAFF_NOTE_EDIT_WINDOW_IN_HOURS)
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserCreatedStaffNote {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EditedUserStaffNote {
    pub content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note_written_at(created_at: DateTime<Utc>, author_id: i32) -> UserStaffNoteWithAuthor {
        UserStaffNoteWithAuthor {
            id: 1,
            user_id: 2,
            created_at,
            content: "note".to_string(),
            author_id,
            author_username: "author".to_string(),
        }
    }

    #[test]
    fn test_permission_allows_editing_any_note() {
        let note = note_written_at(Utc::now() - chrono::Duration::days(365), 3);

        assert!(note.can_be_edited_by(4, true));
    }

    #[test]
    fn test_author_can_edit_recent_note() {
        let note = note_written_at(Utc::now() - chrono::Duration::hours(23), 3);

        assert!(note.can_be_edited_by(3, false));
    }

    #[test]
    fn test_author_cannot_edit_note_older_than_the_window() {
        let note = note_written_at(Utc::now() - chrono::Duration::hours(25), 3);

        assert!(!note.can_be_edited_by(3, false));
    }

    #[test]
    fn test_other_user_cannot_edit_note() {
        let note = note_written_at(Utc::now(), 3);

        assert!(!note.can_be_edited_by(4, false));
    }
}
