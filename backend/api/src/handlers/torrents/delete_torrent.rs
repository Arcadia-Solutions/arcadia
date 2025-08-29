use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use serde_json::json;

use crate::{handlers::User, Arcadia};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{models::torrent::TorrentToDelete, redis::RedisPoolInterface};

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
    current_user: User,
) -> Result<HttpResponse> {
    if current_user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }
    let user_url = &arc
        .frontend_url
        .join(&format!("/user/{}", current_user.id))
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
    arc.pool.remove_torrent(&form, current_user.id).await?;

    Ok(HttpResponse::Ok().json(json!({"result": "success"})))
}
