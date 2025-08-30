use actix_multipart::form::MultipartForm;
use actix_web::{web, HttpResponse};

use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use arcadia_common::error::Result;
use arcadia_storage::models::torrent::{Torrent, UploadedTorrent};

#[utoipa::path(
    post,
    operation_id = "Create torrent",
    tag = "Torrent",
    path = "/api/torrents",
    request_body(content = UploadedTorrent, content_type = "multipart/form-data"),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 201, description = "Successfully uploaded the torrent", body=Torrent),
    )
)]
pub async fn exec(
    form: MultipartForm<UploadedTorrent>,
    arc: web::Data<Arcadia>,
    user: Authdata,
) -> Result<HttpResponse> {
    // TODO : check if user can upload

    let torrent = arc.pool.create_torrent(&form, user.sub).await?;

    Ok(HttpResponse::Created().json(torrent))
}
