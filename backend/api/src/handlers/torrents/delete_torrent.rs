use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use serde_json::json;

use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{torrent::TorrentToDelete, user::UserClass},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    delete,
    operation_id = "Delete torrent",
    tag = "Torrent",
    path = "/api/torrents",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Torrent deleted"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    mut form: Json<TorrentToDelete>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if user.class != UserClass::Staff {
        return Err(Error::InsufficientPrivileges);
    }

    let current_user = arc.pool.find_user_with_id(user.sub).await?;
    let user_url = &arc
        .frontend_url
        .join(&format!("/user/{}", user.sub))
        .unwrap();
    let displayed_reason = format!(
        "A torrent you were a seeder on, has been deleted.
  Please remove it from your torrent client.

Reason: {}

Handled by: [url={}]{}[/url]",
        &form.reason,
        &user_url.as_str(),
        current_user.username
    );

    form.displayed_reason = Some(displayed_reason);
    arc.pool.remove_torrent(&form, user.sub).await?;

    Ok(HttpResponse::Ok().json(json!({"result": "success"})))
}
