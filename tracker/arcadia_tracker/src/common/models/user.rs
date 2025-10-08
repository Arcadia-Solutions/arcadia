use std::ops::{Deref, DerefMut};

use indexmap::IndexMap;
use serde::Serialize;

pub use arcadia_shared::tracker::models::user::{Passkey, User};

#[derive(Serialize)]
pub struct Map(IndexMap<u32, User>);

impl Map {
    pub async fn from_backend() -> Self {
        let base_url =
            std::env::var("ARCADIA_API_BASE_URL").expect("env var ARCADIA_API_BASE_URL not set");
        let url = format!("{}/api/tracker/users", base_url);

        let resp = reqwest::get(url).await.expect("failed to fetch users");
        let bytes = resp
            .bytes()
            .await
            .expect("failed to read users response body");

        let users: Vec<User> = serde_bencode::from_bytes(&bytes).expect("failed to decode users");
        let mut map = IndexMap::new();
        for user in users {
            map.insert(user.id, user);
        }
        Self(map)
    }
}

impl Deref for Map {
    type Target = IndexMap<u32, User>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for Map {
    fn default() -> Self {
        Self(IndexMap::new())
    }
}
