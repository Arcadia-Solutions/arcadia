use serde::Deserialize;
use strum::{Display, EnumString};

#[derive(Clone, Copy, Deserialize, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct InfoHash(pub [u8; 20]);

#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PeerId(pub [u8; 20]);

#[derive(Clone, Copy, PartialEq, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Event {
    #[strum(to_string = "completed")]
    Completed,
    #[strum(to_string = "")]
    Empty,
    #[strum(to_string = "started")]
    Started,
    #[strum(to_string = "stopped")]
    Stopped,
}

pub struct Announce {
    info_hash: InfoHash,
    peer_id: PeerId,
    port: u16,
    uploaded: u64,
    downloaded: u64,
    left: u64,
    event: Event,
    numwant: usize,
    corrupt: Option<u64>,
    key: Option<String>,
}
