use actix_web::{web, HttpResponse};

use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::torrent::{EditedTorrent, Torrent};

#[utoipa::path(
    put,
    operation_id = "Edit torrent",
    tag = "Torrent",
    path = "/api/torrents",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully edited the torrent", body=Torrent),
    )
)]
pub async fn exec(
    form: web::Json<EditedTorrent>,
    arc: web::Data<Arcadia>,
    user: Authdata,
) -> Result<HttpResponse> {
    let torrent = arc.pool.find_torrent(form.id).await?;

    if torrent.created_by_id == user.sub || user.class == "staff" {
        let updated_torrent = arc.pool.update_torrent(&form, torrent.id).await?;
        Ok(HttpResponse::Ok().json(updated_torrent))
    } else {
        Err(Error::InsufficientPrivileges)
    }
}
