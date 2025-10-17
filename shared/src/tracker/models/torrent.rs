use bincode::config;
use reqwest::Client;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, bincode::Encode, bincode::Decode)]
pub struct InfoHash(pub [u8; 20]);

#[derive(Debug, Clone, bincode::Encode, bincode::Decode, PartialEq)]
pub struct Torrent {
    pub upload_factor: f64,
    pub download_factor: f64,
    pub seeders: i64,
    pub leechers: i64,
    pub completed: i64,
}

#[derive(Debug, bincode::Encode, bincode::Decode)]
pub struct Map(HashMap<u32, Torrent>);

impl Map {
    pub async fn from_backend() -> Self {
        let base_url =
            std::env::var("ARCADIA_API_BASE_URL").expect("env var ARCADIA_API_BASE_URL not set");
        let url = format!("{}/api/tracker/torrents", base_url);

        let client = Client::new();
        let api_key = std::env::var("API_KEY").expect("env var API_KEY not set");
        let resp = client
            .get(url)
            .header("api_key", api_key)
            .send()
            .await
            .expect("failed to fetch torrents");
        let bytes = resp
            .bytes()
            .await
            .expect("failed to read torrents response body");

        let config = config::standard();
        let (map, _): (Map, usize) = bincode::decode_from_slice(&bytes[..], config).unwrap();
        map
    }
}

impl Deref for Map {
    type Target = HashMap<u32, Torrent>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
