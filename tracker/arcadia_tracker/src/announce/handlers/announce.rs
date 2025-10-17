use std::future::{self, Ready};

use crate::announce::{error::AnnounceError, models::torrent::Announce};
use actix_web::{dev, web::Path, FromRequest, HttpRequest, HttpResponse, ResponseError};

#[derive(Debug)]
pub struct UserAgent(pub String);

impl std::ops::Deref for UserAgent {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserAgentExtractError {
    #[error("no user agent")]
    NoUserAgent,

    #[error("not decodable as utf-8")]
    ToStrError(#[from] actix_web::http::header::ToStrError),
}

impl ResponseError for UserAgentExtractError {
    fn error_response(&self) -> HttpResponse {
        log::error!("The request generated this error: {self}");
        HttpResponse::BadRequest().body(format!("{self}"))
    }
}

impl FromRequest for UserAgent {
    type Error = UserAgentExtractError;
    type Future = Ready<std::result::Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let agent = req
            .headers()
            .get("User-Agent")
            .ok_or(UserAgentExtractError::NoUserAgent)
            .and_then(|s| Ok(UserAgent(String::from(s.to_str()?))));

        future::ready(agent)
    }
}

#[utoipa::path(
    post,
    operation_id = "Announce",
    tag = "Announce",
    path = "/{passkey}/announce",
    responses(
        (status = 200, description = "Announce"),
    )
)]
pub async fn exec(
    // arc: Data<Tracker>,
    passkey: Path<String>,
    // agent: Option<UserAgent>,
    ann: Announce,
    // conn: dev::ConnectionInfo,
) /*->Result<HttpResponse>*/
{
    println!("{:?}", ann);
    let passkey = u128::from_str_radix(&passkey, 16).map_err(|_| AnnounceError::InvalidPassKey);
    println!("{:?}", passkey);
    //?;

    // Ok(HttpResponse::Ok())
}
