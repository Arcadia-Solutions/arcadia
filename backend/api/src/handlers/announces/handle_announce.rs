use crate::services::announce_service::is_torrent_client_allowed;
use crate::Arcadia;
use actix_web::{dev, web, FromRequest, HttpRequest, HttpResponse, ResponseError};
use arcadia_common::{
    actix::HttpResponseBuilderExt,
    error::announce::Error as AnnounceError,
    models::tracker::announce::{Announce, AnnounceResponse, TorrentEvent},
};
use arcadia_storage::sqlx::types::ipnetwork::IpNetwork;
use std::future::{self, Ready};

type Result<T> = std::result::Result<T, AnnounceError>;

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
        let user_agent = req
            .headers()
            .get("User-Agent")
            .ok_or(UserAgentExtractError::NoUserAgent)
            .and_then(|s| Ok(UserAgent(String::from(s.to_str()?))));

        future::ready(user_agent)
    }
}

pub async fn exec(
    arc: web::Data<Arcadia>,
    passkey: web::Path<String>,
    user_agent: Option<UserAgent>,
    ann: Announce,
    conn: dev::ConnectionInfo,
) -> Result<HttpResponse> {
    if !is_torrent_client_allowed(&ann.peer_id, &arc.tracker.allowed_torrent_clients.clients) {
        return Err(AnnounceError::TorrentClientNotInWhitelist);
    }

    let passkey = u128::from_str_radix(&passkey, 16).map_err(|_| AnnounceError::InvalidPassKey)?;

    let passkey_upper = (passkey >> 64) as i64;
    let passkey_lower = passkey as i64;

    let current_user = arc
        .pool
        .find_user_with_passkey(passkey_upper, passkey_lower)
        .await?;

    let torrent = arc.pool.find_torrent_with_id(&ann.info_hash).await?;

    let ip = conn
        .realip_remote_addr()
        .and_then(|ip| ip.parse::<IpNetwork>().ok())
        .unwrap();

    if let Some(TorrentEvent::Stopped) = ann.event {
        arc.pool
            .remove_peer(&torrent.id, &ann.peer_id, &ip, ann.port)
            .await;
        //return HttpResponse::Ok().into();
        todo!();
    }

    if let Some(TorrentEvent::Completed) = ann.event {
        let _ = arc.pool.increment_torrent_completed(torrent.id).await;
    }

    let (old_real_uploaded, old_real_downloaded) = arc
        .pool
        .insert_or_update_peer(
            &torrent.id,
            &ip,
            &current_user.id,
            &ann,
            user_agent.as_deref(),
        )
        .await;

    let peers = arc
        .pool
        .find_torrent_peers(&torrent.id, &current_user.id)
        .await;

    // assuming that the client either sends both downloaded/uploaded
    // or none of them
    if let (Some(real_uploaded), Some(real_downloaded)) = (ann.uploaded, ann.downloaded) {
        let upload_factor = if arc.tracker.global_upload_factor != 1.0 {
            arc.tracker.global_upload_factor
        } else {
            torrent.upload_factor
        };
        let upload_to_credit =
            ((real_uploaded as i64 - old_real_uploaded) as f64 * upload_factor).ceil() as i64;

        let download_factor = if arc.tracker.global_download_factor != 1.0 {
            arc.tracker.global_download_factor
        } else {
            torrent.download_factor
        };
        let download_to_credit =
            ((real_downloaded as i64 - old_real_downloaded) as f64 * download_factor).ceil() as i64;
        let real_uploaded_to_credit = real_uploaded as i64 - old_real_uploaded;
        let real_downloaded_to_credit = real_downloaded as i64 - old_real_downloaded;

        // if the client restarted, without sending a "stop" event, keeping the same ip/port
        // calculated upload/download might be negative
        if real_uploaded_to_credit >= 0 && real_downloaded_to_credit >= 0 {
            let _ = arc
                .pool
                .credit_user_upload_download(
                    upload_to_credit,
                    download_to_credit,
                    real_uploaded_to_credit,
                    real_downloaded_to_credit,
                    current_user.id,
                )
                .await;
        }
    }

    if ann.left == Some(0u64) {
        let _ = arc
            .pool
            .update_total_seedtime(
                current_user.id,
                torrent.id,
                arc.tracker.announce_interval,
                arc.tracker.announce_interval_grace_period,
            )
            .await;
    }

    let resp = AnnounceResponse {
        peers,
        interval: arc.tracker.announce_interval,
        ..Default::default()
    };

    Ok(HttpResponse::Ok().bencode(resp))
}
