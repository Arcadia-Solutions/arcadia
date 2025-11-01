use crate::tracker::models::user::Passkey;
use indexmap::IndexMap;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Map(pub IndexMap<Passkey, u32>);

impl Deref for Map {
    type Target = IndexMap<Passkey, u32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct DBImportPasskey2Id {
    pub id: i32,
    pub passkey: Passkey,
}

impl Map {
    pub async fn from_database(db: &sqlx::PgPool) -> Self {
        let rows = sqlx::query_as!(
            DBImportPasskey2Id,
            r#"
                    SELECT
                        id,
                        passkey as "passkey: Passkey"
                    FROM users
                    WHERE banned = FALSE
                "#
        )
        .fetch_all(db)
        .await
        .expect("could not get passkeys2ids");

        let mut map: Map = Map(IndexMap::with_capacity(rows.len()));
        for r in rows {
            map.insert(r.passkey, r.id as u32);
        }

        map
    }
}
