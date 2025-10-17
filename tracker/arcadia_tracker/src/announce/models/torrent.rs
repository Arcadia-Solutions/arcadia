use actix_web::{dev, FromRequest, HttpRequest};
use arcadia_shared::tracker::models::torrent::InfoHash;
use std::{
    borrow::Cow,
    future::{self, Ready},
    str::FromStr,
};

use crate::announce::error::AnnounceError;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PeerId(pub [u8; 20]);

#[derive(Debug)]
pub struct Announce {
    #[allow(dead_code)]
    info_hash: InfoHash,
    #[allow(dead_code)]
    peer_id: PeerId,
    #[allow(dead_code)]
    port: u16,
    #[allow(dead_code)]
    uploaded: Option<u64>,
    #[allow(dead_code)]
    downloaded: Option<u64>,
    #[allow(dead_code)]
    left: Option<u64>,
    #[allow(dead_code)]
    event: Option<TorrentEvent>,
    #[allow(dead_code)]
    numwant: Option<usize>,
    // corrupt: Option<u64>,
    // key: Option<String>,
    #[allow(dead_code)]
    pub compact: Option<bool>,
    #[allow(dead_code)]
    pub ip: Option<std::net::Ipv4Addr>,
}

impl FromRequest for Announce {
    type Error = AnnounceError;
    type Future = Ready<std::result::Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let query_string = req.query_string();

        let announce = decode_from_query_str(query_string);

        future::ready(announce)
    }
}

#[derive(Debug, PartialEq)]
pub enum TorrentEvent {
    Started,
    Stopped,
    Completed,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum ParseTorrentEventError {
    #[error("unknown event")]
    UnknownEvent,
}

impl FromStr for TorrentEvent {
    type Err = ParseTorrentEventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "started" => Ok(TorrentEvent::Started),
            "stopped" => Ok(TorrentEvent::Stopped),
            "completed" => Ok(TorrentEvent::Completed),
            _ => Err(ParseTorrentEventError::UnknownEvent),
        }
    }
}

struct QueryPairs<'a> {
    input: &'a str,
}

impl<'a> Iterator for QueryPairs<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.input.is_empty() {
                return None;
            }

            let mut split2 = self.input.splitn(2, '&');
            let sequence = split2.next().unwrap();
            self.input = split2.next().unwrap_or_default();
            if sequence.is_empty() {
                continue;
            }
            let mut split2 = sequence.splitn(2, '=');
            let name = split2.next().unwrap();
            let value = split2.next().unwrap_or_default();
            return Some((name, value));
        }
    }
}

struct IncorrectLength;

fn decode_into_20_byte_array(value: &str) -> Result<[u8; 20], IncorrectLength> {
    let x = Cow::from(percent_encoding::percent_decode_str(value));

    if x.len() != 20 {
        return Err(IncorrectLength);
    }

    Ok(std::array::from_fn(|i| unsafe {
        // SAFETY: length checked above.
        *x.get_unchecked(i)
    }))
}

pub fn decode_from_query_str(query: &str) -> Result<Announce, AnnounceError> {
    let mut info_hash = Option::<[u8; 20]>::None;
    let mut peer_id = Option::<[u8; 20]>::None;
    let mut ip = Option::<std::net::Ipv4Addr>::None;
    let mut port = Option::<u16>::None;
    let mut uploaded = Option::<u64>::None;
    let mut downloaded = Option::<u64>::None;
    let mut left = Option::<u64>::None;
    let mut event = Option::<TorrentEvent>::None;
    let mut compact = Option::<bool>::None;
    let mut numwant = Option::<usize>::None;

    let pairs = QueryPairs { input: query };

    for (name, value) in pairs {
        match name {
            "info_hash" => {
                info_hash = Some(
                    decode_into_20_byte_array(value).map_err(|_| AnnounceError::InvalidInfoHash)?,
                );
            }
            "peer_id" => {
                peer_id = Some(
                    decode_into_20_byte_array(value).map_err(|_| AnnounceError::InvalidPeerId)?,
                );
            }
            "port" => {
                port = Some(u16::from_str(value).map_err(AnnounceError::InvalidPort)?);
            }
            "uploaded" => {
                uploaded = Some(u64::from_str(value).map_err(AnnounceError::InvalidUploaded)?);
            }
            "downloaded" => {
                downloaded = Some(u64::from_str(value).map_err(AnnounceError::InvalidDownloaded)?);
            }
            "left" => {
                left = Some(u64::from_str(value).map_err(AnnounceError::InvalidLeft)?);
            }
            "event" => {
                event = Some(TorrentEvent::from_str(value).map_err(AnnounceError::InvalidEvent)?);
            }
            "compact" => match value {
                "1" => compact = Some(true),
                "0" => return Err(AnnounceError::UnsupportedCompact),
                _ => return Err(AnnounceError::InvalidCompact),
            },
            "ip" => {
                ip = Some(
                    std::net::Ipv4Addr::from_str(value).map_err(AnnounceError::InvalidIpAddr)?,
                );
            }
            "numwant" => {
                numwant = Some(usize::from_str(value).map_err(AnnounceError::InvalidNumWant)?);
            }

            // key?
            _ => continue,
        };
    }

    Ok(Announce {
        info_hash: InfoHash(info_hash.ok_or(AnnounceError::MissingInfoHash)?),
        peer_id: PeerId(peer_id.ok_or(AnnounceError::MissingPeerId)?),
        ip,
        port: port.ok_or(AnnounceError::MissingPort)?,
        uploaded,
        downloaded,
        left,
        event,
        compact,
        numwant,
    })
}
