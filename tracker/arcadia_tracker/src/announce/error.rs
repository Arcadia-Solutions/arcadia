use serde::Serialize;

use crate::announce::HttpResponseBuilderExt;

pub type Result<T> = std::result::Result<T, AnnounceError>;

#[derive(Debug, thiserror::Error)]
pub enum AnnounceError {
    #[error("Internal tracker error.")]
    InternalTrackerError,
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
    #[error("invalid ip")]
    InvalidIpAddr(#[source] std::net::AddrParseError),
    #[error("invalid numwant")]
    InvalidNumWant(#[source] std::num::ParseIntError),
    #[error("invalid compact")]
    InvalidCompact,
    #[error("only compact=1 supported")]
    UnsupportedCompact,
    #[error("Abnormal access blocked.")]
    AbnormalAccess,
    #[error("user-agent is missing")]
    NoUserAgent,
    #[error("not decodable as utf-8")]
    ToStrError(#[from] actix_web::http::header::ToStrError),
    #[error("The user agent of this client is too long.")]
    UserAgentTooLong,
    #[error("Passkey does not exist. Please re-download the .torrent file.")]
    PasskeyNotFound,
    #[error("Invalid passkey.")]
    InvalidPasskey,
    #[error("User does not exist. Please re-download the .torrent file.")]
    UserNotFound,
    #[error("InfoHash not found.")]
    InfoHashNotFound,
    #[error("Unsupported 'event' type.")]
    UnsupportedEvent,
    #[error("invalid event")]
    InvalidEvent,
    #[error("Torrent not found.")]
    TorrentNotFound,
    #[error("Torrent has been deleted.")]
    TorrentIsDeleted,
    #[error("Query parameter 'left' is missing.")]
    MissingLeft,
    #[error("Missing IP address in query")]
    MissingIpAddr,
    #[error("Rate limit exceeded. Please wait.")]
    RateLimitExceeded,
    #[error("You already have {0} peers on this torrent. Ignoring.")]
    PeersPerTorrentPerUserLimit(u8),
    #[error("You have already leeched {0} torrents in the past 24h.")]
    SnatchLimitReached(u32),
    #[error("Uploaded value is missing.")]
    MissingUploaded,
    #[error("Downloaded value is missing.")]
    MissingDownloaded,
    #[error("Not enough bonus points to download this torrent (cost: {0} BP).")]
    InsufficientBonusPoints(i64),
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
