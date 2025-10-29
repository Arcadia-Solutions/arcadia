use actix_multipart::form::MultipartForm;
use actix_web::{web::Data, HttpRequest, HttpResponse};

use crate::{middlewares::auth_middleware::Authdata, Arcadia};
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
    mut form: MultipartForm<UploadedTorrent>,
    req: HttpRequest,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    // TODO : check if user can upload

    // Check for X-Upload-Method header to support upload tools
    if let Some(header_value) = req.headers().get("X-Upload-Method") {
        // Update the upload_method field from header value
        if let Ok(method_str) = header_value.to_str() {
            form.upload_method = actix_multipart::form::text::Text(method_str.to_string());
        }
    }

    let torrent = arc.pool.create_torrent(&form, user.sub).await?;

    Ok(HttpResponse::Created().json(torrent))
}
