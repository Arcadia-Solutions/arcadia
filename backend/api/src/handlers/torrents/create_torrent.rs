use actix_multipart::form::MultipartForm;
use actix_web::{
    web::{self, Data},
    HttpResponse,
};

use crate::{handlers::User, Arcadia};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::torrent::{Torrent, UploadedTorrent},
    redis::RedisPoolInterface,
};

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
pub async fn exec<R: RedisPoolInterface + 'static>(
    form: MultipartForm<UploadedTorrent>,
    arc: Data<Arcadia<R>>,
    current_user: User,
) -> Result<HttpResponse> {
    // TODO : check if user can upload

    let torrent = arc.pool.create_torrent(&form, &current_user).await?;

    Ok(HttpResponse::Created().json(torrent))
}
