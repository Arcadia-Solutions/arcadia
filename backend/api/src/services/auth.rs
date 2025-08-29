use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserBlacklistEntry {
    user_id: i64,
    token_invalidation_ts: i64,
}

impl UserBlacklistEntry {
    pub fn new(user_id: i64) -> Self {
        let now = Utc::now();

        Self {
            user_id,
            token_invalidation_ts: now.timestamp(),
        }
    }
}
