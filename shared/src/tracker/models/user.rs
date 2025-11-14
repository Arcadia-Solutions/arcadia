use anyhow::bail;
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::{Database, Decode, PgPool};
use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Passkey(pub [u8; 32]);

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct User {
    pub num_seeding: u32,
    pub num_leeching: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct APIInsertUser {
    pub id: u32,
    pub passkey: Passkey,
}

#[derive(Debug, Serialize)]
pub struct Map(pub IndexMap<u32, User>);

impl FromStr for Passkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = s.bytes();

        if bytes.len() != 32 {
            bail!("Invalid passkey length.");
        }

        let array = [(); 32].map(|_| bytes.next().unwrap());

        Ok(Passkey(array))
    }
}

impl<'r, DB: Database> Decode<'r, DB> for Passkey
where
    &'r str: Decode<'r, DB>,
{
    fn decode(
        value: <DB as Database>::ValueRef<'r>,
    ) -> Result<Passkey, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <&str as Decode<DB>>::decode(value)?;
        let mut bytes = value.bytes();

        let array = [(); 32].map(|_| bytes.next().expect("Invalid passkey length."));

        Ok(Passkey(array))
    }
}

impl Display for Passkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&String::from_utf8_lossy(&self.0))
    }
}

impl Serialize for Passkey {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Passkey {
    fn deserialize<D>(deserializer: D) -> std::prelude::v1::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
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

#[derive(Debug)]
pub struct DBImportUser {
    pub id: i32,
    pub passkey: Passkey,
    pub num_seeding: i32,
    pub num_leeching: i32,
}

impl Map {
    pub async fn from_database(db: &PgPool) -> Self {
        let rows = sqlx::query_as!(
            DBImportUser,
            r#"
            SELECT
                id,
                passkey as "passkey: Passkey",
                0::INT AS "num_seeding!",
                0::INT AS "num_leeching!"
            FROM users
            "#
        )
        .fetch_all(db)
        .await
        .expect("could not get users");

        let mut map: Map = Map(IndexMap::with_capacity(rows.len()));
        for r in rows {
            let user = User {
                num_seeding: r.num_seeding as u32,
                num_leeching: r.num_leeching as u32,
            };
            map.insert(r.id as u32, user);
        }

        map
    }
}
