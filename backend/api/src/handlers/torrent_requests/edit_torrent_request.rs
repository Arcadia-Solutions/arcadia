use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_storage::{
    models::{
        torrent_request::{EditedTorrentRequest, TorrentRequest},
        user::UserClass,
    },
    redis::RedisPoolInterface,
};

use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use arcadia_common::error::{Error, Result};

#[utoipa::path(
    put,
    operation_id = "Edit torrent request",
    tag = "Torrent Request",
    path = "/api/torrent-requests",
    security(
        ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully edited the torrent request", body=TorrentRequest),
    )
)]

pub async fn exec<R: RedisPoolInterface + 'static>(
    form: Json<EditedTorrentRequest>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let torrent_request = arc.pool.find_torrent_request(form.id).await?;

    if user.class == UserClass::Staff || torrent_request.created_by_id == user.sub {
        let updated_torrent_request = arc
            .pool
            .update_torrent_request(&form, torrent_request.id)
            .await?;
        Ok(HttpResponse::Ok().json(updated_torrent_request))
    } else {
        Err(Error::InsufficientPrivileges)
    }
}
