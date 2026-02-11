use actix_web::HttpResponse;

pub type Result<T> = std::result::Result<T, AnnounceError>;

#[derive(Debug, thiserror::Error, strum::AsRefStr)]
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
    #[error("Stopped peer doesn't exist.")]
    StoppedPeerDoesNotExist,
}

impl actix_web::ResponseError for AnnounceError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::OK
    }
    fn error_response(&self) -> HttpResponse {
        let mut response: Vec<u8> = vec![];

        if self.is_critical_warning() {
            response.extend(b"d8:completei0e10:downloadedi0e10:incompletei0e");

            response.extend(b"8:intervali5400e12:min intervali5400e");

            response.extend(b"5:peers0:15:warning message");
            response.extend(self.to_string().len().to_string().as_bytes());
            response.extend(b":");
            response.extend(self.to_string().as_bytes());
            response.extend(b"e");
        } else {
            response.extend(b"d14:failure reason");
            response.extend(self.to_string().len().to_string().as_bytes());
            response.extend(b":");
            response.extend(self.to_string().as_bytes());

            response.extend(b"8:intervali5400e12:min intervali5400ee");
        }

        // (StatusCode::OK, response).into_response()
        actix_web::HttpResponse::build(self.status_code()).body(response)
    }
}

impl AnnounceError {
    /// Announce warnings that act as an error by immediately returning
    /// an empty peer list but are not explicit errors due to undesired
    /// side effects.
    fn is_critical_warning(&self) -> bool {
        match self {
            // Some clients (namely transmission) will keep sending
            // `stopped` events until a successful announce is received.
            // If a user's network is having issues, their peer might be
            // deleted for inactivity from missed announces. If their peer
            // isn't found when we receive a `stopped` event from them
            // after regaining network connectivity, we can't return an
            // error otherwise the client might enter into an infinite loop
            // of sending `stopped` events. To prevent this, we need to
            // send a warning (i.e. succcessful announce) instead, so that
            // the client can successfully restart its session.
            Self::StoppedPeerDoesNotExist => true,
            _ => false,
        }
    }
}
