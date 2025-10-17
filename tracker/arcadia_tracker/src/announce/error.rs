use serde::Serialize;

use crate::announce::{models::torrent::ParseTorrentEventError, HttpResponseBuilderExt};

pub type Result<T> = std::result::Result<T, AnnounceError>;

#[derive(Debug, thiserror::Error)]
pub enum AnnounceError {
    #[error("invalid passkey")]
    InvalidPassKey,

    #[error("invalid info_hash")]
    InvalidInfoHash,

    #[error("invalid user id")]
    InvalidUserId,

    #[error("invalid peer id")]
    InvalidPeerId,

    #[error("invalid user id or torrent id")]
    InvalidUserIdOrTorrentId,

    #[error("torrent client not in whitelist")]
    TorrentClientNotInWhitelist,

    #[error("missing info_hash")]
    MissingInfoHash,

    #[error("missing peer_id")]
    MissingPeerId,

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

impl actix_web::ResponseError for AnnounceError {
    #[inline]
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        log::error!("The request generated this error: {self}");
        #[derive(Debug, Serialize)]
        struct WrappedError {
            #[serde(rename = "failure reason")]
            failure_reason: String,
        }

        actix_web::HttpResponse::build(self.status_code()).bencode(WrappedError {
            failure_reason: self.to_string(),
        })
    }
}
