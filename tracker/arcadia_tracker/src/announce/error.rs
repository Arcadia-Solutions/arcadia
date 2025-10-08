use serde::Serialize;

use crate::announce::HttpResponseBuilderExt;

pub type Result<T> = std::result::Result<T, AnnounceError>;

#[derive(Debug, thiserror::Error)]
pub enum AnnounceError {
    #[error("invalid passkey")]
    InvalidPassKey,

    #[error("invalid info_hash")]
    InvalidInfoHash,

    #[error("invalid user id")]
    InvalidUserId,

    #[error("invalid user id or torrent id")]
    InvalidUserIdOrTorrentId,

    #[error("torrent client not in whitelist")]
    TorrentClientNotInWhitelist,
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
