use actix_web::{dev, FromRequest, HttpRequest, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use std::future::Ready;
use std::str::FromStr;
use std::{borrow::Cow, future};

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

#[derive(Debug)]
pub struct Announce {
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
    // TODO: use this
    #[allow(dead_code)]
    pub ip: Option<std::net::Ipv4Addr>,
    pub port: u16,
    pub uploaded: Option<u64>,
    pub downloaded: Option<u64>,
    pub left: Option<u64>,
    pub event: Option<TorrentEvent>,
    // TODO: use this
    #[allow(dead_code)]
    pub compact: Option<bool>,
    // TODO: use this
    #[allow(dead_code)]
    pub numwant: Option<u64>,
}

impl FromRequest for Announce {
    type Error = Error;
    type Future = Ready<std::result::Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let query_string = req.query_string();

        let announce = decode_from_query_str(query_string);

        future::ready(announce)
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("missing info_hash")]
    MissingInfoHash,

    #[error("invalid info_hash")]
    InvalidInfoHash,

    #[error("missing peer_id")]
    MissingPeerId,

    #[error("invalid peer_id")]
    InvalidPeerId,

    #[error("missing port")]
    MissingPort,

    #[error("invalid port")]
    InvalidPort(#[source] std::num::ParseIntError),

    #[error("invalid uploaded")]
    InvalidUploaded(#[source] std::num::ParseIntError),

    #[error("invalid downloaded")]
    InvalidDownloaded(#[source] std::num::ParseIntError),

    #[error("invalid left")]
    InvalidLeft(#[source] std::num::ParseIntError),

    #[error("invalid event")]
    InvalidEvent(#[source] ParseTorrentEventError),

    #[error("invalid ip")]
    InvalidIpAddr(#[source] std::net::AddrParseError),

    #[error("invalid numwant")]
    InvalidNumWant(#[source] std::num::ParseIntError),

    #[error("invalid compact")]
    InvalidCompact,

    #[error("only compact=1 supported")]
    UnsupportedCompact,
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        log::error!("The request generated this error: {self}");
        HttpResponse::BadRequest().body(format!("{self}"))
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

pub fn decode_from_query_str(query: &str) -> Result<Announce, Error> {
    let mut info_hash = Option::<[u8; 20]>::None;
    let mut peer_id = Option::<[u8; 20]>::None;
    let mut ip = Option::<std::net::Ipv4Addr>::None;
    let mut port = Option::<u16>::None;
    let mut uploaded = Option::<u64>::None;
    let mut downloaded = Option::<u64>::None;
    let mut left = Option::<u64>::None;
    let mut event = Option::<TorrentEvent>::None;
    let mut compact = Option::<bool>::None;
    let mut numwant = Option::<u64>::None;

    let pairs = QueryPairs { input: query };

    for (name, value) in pairs {
        match name {
            "info_hash" => {
                info_hash =
                    Some(decode_into_20_byte_array(value).map_err(|_| Error::InvalidInfoHash)?);
            }
            "peer_id" => {
                peer_id = Some(decode_into_20_byte_array(value).map_err(|_| Error::InvalidPeerId)?);
            }
            "port" => {
                port = Some(u16::from_str(value).map_err(Error::InvalidPort)?);
            }
            "uploaded" => {
                uploaded = Some(u64::from_str(value).map_err(Error::InvalidUploaded)?);
            }
            "downloaded" => {
                downloaded = Some(u64::from_str(value).map_err(Error::InvalidDownloaded)?);
            }
            "left" => {
                left = Some(u64::from_str(value).map_err(Error::InvalidLeft)?);
            }
            "event" => {
                event = Some(TorrentEvent::from_str(value).map_err(Error::InvalidEvent)?);
            }
            "compact" => match value {
                "1" => compact = Some(true),
                "0" => return Err(Error::UnsupportedCompact),
                _ => return Err(Error::InvalidCompact),
            },
            "ip" => {
                ip = Some(std::net::Ipv4Addr::from_str(value).map_err(Error::InvalidIpAddr)?);
            }
            "numwant" => {
                numwant = Some(u64::from_str(value).map_err(Error::InvalidNumWant)?);
            }

            // key?
            _ => continue,
        };
    }

    Ok(Announce {
        info_hash: info_hash.ok_or(Error::MissingInfoHash)?,
        peer_id: peer_id.ok_or(Error::MissingPeerId)?,
        ip,
        port: port.ok_or(Error::MissingPort)?,
        uploaded,
        downloaded,
        left,
        event,
        compact,
        numwant,
    })
}

#[derive(Debug, PartialEq)]
pub struct Peer {
    pub ip: std::net::Ipv4Addr,
    pub port: u16,
}

pub mod peer_list {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(peers: &Vec<Peer>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut vec = Vec::with_capacity(peers.len() * 6);
        for peer in peers {
            vec.extend_from_slice(&peer.ip.octets());
            vec.extend_from_slice(&peer.port.to_be_bytes());
        }
        serializer.serialize_bytes(&vec)
    }

    struct BytesVisitor;

    impl serde::de::Visitor<'_> for BytesVisitor {
        type Value = Vec<Peer>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("bytestring with multiple of 6 bytes")
        }

        fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if (bytes.len() % 6) != 0 {
                return Err(E::invalid_length(
                    bytes.len(),
                    &"bytestring with multiple of 6 bytes",
                ));
            }

            let num_peers = bytes.len() / 6;

            let mut vec = Vec::with_capacity(num_peers);

            for i in 0..num_peers {
                let ip =
                    std::net::Ipv4Addr::new(bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]);
                let port = u16::from_be_bytes([bytes[i + 4], bytes[i + 5]]);
                vec.push(Peer { ip, port })
            }

            Ok(vec)
        }
    }

    pub fn deserialize<'de, D>(deser: D) -> Result<Vec<Peer>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deser.deserialize_bytes(BytesVisitor)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AnnounceResponse {
    pub interval: u32,

    #[serde(rename = "min interval")]
    pub min_interval: u32,

    #[serde(with = "peer_list")]
    pub peers: Vec<Peer>,
}

impl Default for AnnounceResponse {
    #[inline]
    fn default() -> Self {
        AnnounceResponse {
            interval: 1800,
            min_interval: 30,
            peers: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_announce_response() {
        let resp = AnnounceResponse {
            interval: 1800,
            min_interval: 30,
            peers: vec![Peer {
                ip: std::net::Ipv4Addr::new(10, 10, 10, 5),
                port: 128,
            }],
        };

        let enc = serde_bencode::to_bytes(&resp);

        assert!(enc.is_ok());

        let enc = enc.unwrap();
        assert_eq!(
            &enc,
            &b"d8:intervali1800e12:min intervali30e5:peers6:\x0A\x0A\x0A\x05\x00\x80e"
        );
    }

    #[test]
    fn test_deserialize_announce_response() {
        let data = b"d8:intervali1800e12:min intervali30e5:peers6:\x0A\x0A\x0A\x05\x00\x80e";

        let peer = serde_bencode::de::from_bytes::<AnnounceResponse>(data)
            .expect("valid data should be deserializable");

        assert_eq!(
            peer,
            AnnounceResponse {
                interval: 1800,
                min_interval: 30,
                peers: vec![Peer {
                    ip: std::net::Ipv4Addr::new(10, 10, 10, 5),
                    port: 128,
                }],
            }
        );
    }

    #[test]
    fn test_serialize_response() {
        let resp = AnnounceResponse {
            interval: 1800,
            min_interval: 30,
            peers: vec![Peer {
                ip: std::net::Ipv4Addr::new(10, 10, 10, 5),
                port: 128,
            }],
        };

        let enc = serde_bencode::to_bytes(&resp);

        assert!(enc.is_ok());

        let enc = enc.unwrap();

        assert_eq!(
            &enc,
            &b"d8:intervali1800e12:min intervali30e5:peers6:\x0A\x0A\x0A\x05\x00\x80e"
        );
    }

    #[test]
    fn test_empty_fails() {
        let announce = decode_from_query_str("");
        assert!(announce.is_err());
    }

    #[test]
    fn test_parse_valid_query() {
        let query = "info_hash=%7C%B3%C6y%9A%FFm%5C%3B%10%A6S%1FF%07%D9%C9%0E%C0%A7&peer_id=-lt0F01-%AB%14%AD%B1%10%ED%F2%E8%7B%E6%24%F3&key=1c597cf5&compact=1&port=6909&uploaded=0&downloaded=1&left=14&numwant=48&event=started";

        let announce = decode_from_query_str(query);

        let announce = announce.expect("expected valid announce query string");

        assert_eq!(
            &announce.info_hash,
            b"\x7C\xB3\xC6y\x9A\xFFm\x5C\x3B\x10\xA6S\x1FF\x07\xD9\xC9\x0E\xC0\xA7"
        );
        assert_eq!(
            &announce.peer_id,
            b"-lt0F01-\xAB\x14\xAD\xB1\x10\xED\xF2\xE8\x7B\xE6\x24\xF3"
        );
        assert_eq!(announce.port, 6909);
        assert_eq!(announce.uploaded, Some(0));
        assert_eq!(announce.downloaded, Some(1));
        assert_eq!(announce.left, Some(14));
        assert_eq!(announce.event, Some(TorrentEvent::Started));
        assert_eq!(announce.compact, Some(true));
        assert_eq!(announce.numwant, Some(48));
    }

    #[test]
    fn test_parse_missing_fields() {
        let query = "peer_id=-lt0F01-%AB%14%AD%B1%10%ED%F2%E8%7B%E6%24%F3&key=1c597cf5&compact=1&port=6909&uploaded=0&downloaded=1&left=14&event=started";
        let announce = decode_from_query_str(query);

        let err = announce.expect_err("expected to fail with missing fields");

        assert_eq!(err, Error::MissingInfoHash);
    }

    #[test]
    fn test_parse_invalid_port() {
        let query = "info_hash=%7C%B3%C6y%9A%FFm%5C%3B%10%A6S%1FF%07%D9%C9%0E%C0%A7&peer_id=-lt0F01-%AB%14%AD%B1%10%ED%F2%E8%7B%E6%24%F3&key=1c597cf5&compact=1&port=invalid&uploaded=0&downloaded=1&left=14&event=started";

        let announce = decode_from_query_str(query);

        let err = announce.expect_err("expected to fail with invalid port");

        assert!(
            matches!(err, Error::InvalidPort(_)),
            "err is not invalid port: {err:?}"
        );
    }

    #[test]
    fn test_parse_invalid_uploaded() {
        let query = "info_hash=%7C%B3%C6y%9A%FFm%5C%3B%10%A6S%1FF%07%D9%C9%0E%C0%A7&peer_id=-lt0F01-%AB%14%AD%B1%10%ED%F2%E8%7B%E6%24%F3&key=1c597cf5&compact=1&port=6909&uploaded=invalid&downloaded=1&left=14&event=started";

        let announce = decode_from_query_str(query);

        let err = announce.expect_err("expected to fail with invalid uploaded");

        assert!(
            matches!(err, Error::InvalidUploaded(_)),
            "err is not invalid uploaded: {err:?}"
        );
    }

    #[test]
    fn test_parse_invalid_downloaded() {
        let query = "info_hash=%7C%B3%C6y%9A%FFm%5C%3B%10%A6S%1FF%07%D9%C9%0E%C0%A7&peer_id=-lt0F01-%AB%14%AD%B1%10%ED%F2%E8%7B%E6%24%F3&key=1c597cf5&compact=1&port=6909&uploaded=0&downloaded=invalid&left=14&event=started";

        let announce = decode_from_query_str(query);

        let err = announce.expect_err("expected to fail with invalid downloaded");

        assert!(
            matches!(err, Error::InvalidDownloaded(_)),
            "err is not invalid downloaded: {err:?}"
        );
    }

    #[test]
    fn test_parse_invalid_left() {
        let query = "info_hash=%7C%B3%C6y%9A%FFm%5C%3B%10%A6S%1FF%07%D9%C9%0E%C0%A7&peer_id=-lt0F01-%AB%14%AD%B1%10%ED%F2%E8%7B%E6%24%F3&key=1c597cf5&compact=1&port=6909&uploaded=0&downloaded=1&left=invalid&event=started";

        let announce = decode_from_query_str(query);

        let err = announce.expect_err("expected to fail with invalid left");

        assert!(
            matches!(err, Error::InvalidLeft(_)),
            "err is not invalid left: {err:?}"
        );
    }

    #[test]
    fn test_parse_invalid_event() {
        let query = "info_hash=%7C%B3%C6y%9A%FFm%5C%3B%10%A6S%1FF%07%D9%C9%0E%C0%A7&peer_id=-lt0F01-%AB%14%AD%B1%10%ED%F2%E8%7B%E6%24%F3&key=1c597cf5&compact=1&port=6909&uploaded=0&downloaded=1&left=14&event=invalid";

        let announce = decode_from_query_str(query);

        let err = announce.expect_err("expected to fail with invalid event");

        assert!(
            matches!(err, Error::InvalidEvent(_)),
            "err is not invalid event: {err:?}"
        );
    }

    #[test]
    fn test_parse_invalid_numwant() {
        let query = "info_hash=%7C%B3%C6y%9A%FFm%5C%3B%10%A6S%1FF%07%D9%C9%0E%C0%A7&peer_id=-lt0F01-%AB%14%AD%B1%10%ED%F2%E8%7B%E6%24%F3&key=1c597cf5&compact=1&port=6909&uploaded=0&downloaded=1&left=14&numwant=invalid&event=started";

        let announce = decode_from_query_str(query);

        let err = announce.expect_err("expected to fail with invalid numwant");

        assert!(
            matches!(err, Error::InvalidNumWant(_)),
            "err is not invalid numwant: {err:?}"
        );
    }
}
