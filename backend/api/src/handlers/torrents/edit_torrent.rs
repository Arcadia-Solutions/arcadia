use actix_web::{
    web::{Data, Json},
    HttpResponse,
};

use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{
        torrent::{EditedTorrent, Torrent},
        user::UserPermission,
        user_edit_change_log::NewUserEditChangeLog,
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

    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::EditTorrent)
        .await?
        && torrent.created_by_id != user.sub
    {
        return Err(Error::InsufficientPermissions(format!(
            "{:?}",
            UserPermission::EditTorrent
        )));
    }

    if let Some(edits) = torrent.diff(&form) {
        arc.pool
            .create_user_edit_change_log(&NewUserEditChangeLog {
                item_type: "torrent".to_string(),
                item_id: torrent.id as i64,
                edited_by_id: user.sub,
                edits,
            })
            .await?;
    }

    let updated_torrent = arc.pool.update_torrent(&form, torrent.id).await?;
    Ok(HttpResponse::Ok().json(updated_torrent))
}
