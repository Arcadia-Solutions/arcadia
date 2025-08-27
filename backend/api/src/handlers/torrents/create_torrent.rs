use actix_multipart::form::MultipartForm;
use actix_web::{web, HttpResponse};

use crate::{handlers::User, Arcadia};
use arcadia_common::error::Result;
use arcadia_storage::models::torrent::{Torrent, UploadedTorrent};

#[utoipa::path(
    post,
    operation_id = "Create torrent",
    tag = "Torrent",
    path = "/api/torrents",
    request_body(content = UploadedTorrent, content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Successfully uploaded the torrent", body=Torrent),
    )
)]
pub async fn exec(
    form: MultipartForm<UploadedTorrent>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    // TODO : check if user can upload

    let torrent = arc.pool.create_torrent(&form, &current_user).await?;

    Ok(HttpResponse::Created().json(torrent))
}
