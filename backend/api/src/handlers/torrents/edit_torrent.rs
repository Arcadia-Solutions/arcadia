use actix_web::{
    web::{Data, Json},
    HttpResponse,
};

use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{
        torrent::{EditedTorrent, Torrent},
        user::UserClass,
    },
    redis::RedisPoolInterface,
};

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
pub async fn exec<R: RedisPoolInterface + 'static>(
    form: Json<EditedTorrent>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let torrent = arc.pool.find_torrent(form.id).await?;

    if user.class != UserClass::Staff && torrent.created_by_id != user.sub {
        return Err(Error::InsufficientPrivileges);
    }

    let updated_torrent = arc.pool.update_torrent(&form, torrent.id).await?;
    Ok(HttpResponse::Ok().json(updated_torrent))
}
