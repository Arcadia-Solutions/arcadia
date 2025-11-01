use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

use serde::{Deserialize, Serialize, Serializer};
use sqlx::{Database, Decode};

use crate::utils::hex_encode;

#[derive(Clone, Copy, Eq, Deserialize, Hash, PartialEq, PartialOrd, Ord)]
pub struct PeerId(pub [u8; 20]);

impl Deref for PeerId {
    type Target = [u8; 20];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<[u8; 20]> for PeerId {
    fn from(array: [u8; 20]) -> Self {
        PeerId(array)
    }
}

impl From<&[u8]> for PeerId {
    fn from(slice: &[u8]) -> Self {
        let peer_id: [u8; 20] = slice.try_into().expect("Invalid peer id.");
        PeerId(peer_id)
    }
}

impl<'r, DB: Database> Decode<'r, DB> for PeerId
where
    &'r [u8]: Decode<'r, DB>,
{
    /// Decodes the database's 2-byte binary peer_id
    fn decode(
        value: <DB as Database>::ValueRef<'r>,
    ) -> Result<PeerId, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let value = <&[u8] as Decode<DB>>::decode(value)?;

        Ok(value.into())
    }
}

impl Display for PeerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hex = [0u8; 40];

        for i in 0..self.0.len() {
            [hex[2 * i], hex[2 * i + 1]] = hex_encode(self.0[i]);
        }

        f.write_str(&String::from_utf8_lossy(&hex))
    }
}

impl Debug for PeerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl Serialize for PeerId {
    fn serialize<S>(&self, serializer: S) -> std::prelude::v1::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
