use crate::tracker::models::torrent::InfoHash;
use indexmap::IndexMap;
use sqlx::PgPool;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Map(pub IndexMap<InfoHash, u32>);

impl Deref for Map {
    type Target = IndexMap<InfoHash, u32>;

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
pub struct DBImportInfohash2Id {
    pub id: i32,
    pub info_hash: InfoHash,
}

impl Map {
    pub async fn from_database(db: &PgPool) -> Self {
        let rows = sqlx::query_as!(
            DBImportInfohash2Id,
            r#"
                    SELECT
                        id,
                        info_hash as "info_hash: InfoHash"
                    FROM torrents
                "#
        )
        .fetch_all(db)
        .await
        .expect("could not get infohashes2ids");

        let mut map: Map = Map(IndexMap::with_capacity(rows.len()));
        for r in rows {
            map.insert(r.info_hash, r.id as u32);
        }

        map
    }
}
