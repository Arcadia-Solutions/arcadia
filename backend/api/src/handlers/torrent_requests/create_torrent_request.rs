use crate::handlers::User;
use crate::Arcadia;
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::torrent_request::{TorrentRequest, UserCreatedTorrentRequest};

#[utoipa::path(
    post,
    operation_id = "Create torrent request",
    tag = "Torrent Request",
    path = "/api/torrent-requests",
    responses(
        (status = 200, description = "Successfully created the torrent_request", body=TorrentRequest),
    )
)]
pub async fn exec(
    mut torrent_request: web::Json<UserCreatedTorrentRequest>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let torrent_request = arc
        .pool
        .create_torrent_request(&mut torrent_request, &current_user)
        .await?;

    Ok(HttpResponse::Created().json(torrent_request))
}
