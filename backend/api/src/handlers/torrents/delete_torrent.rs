use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse,
};

use chrono::Local;
use serde_json::json;

use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{torrent::TorrentToDelete, user::UserPermission},
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
    req: HttpRequest,
) -> Result<HttpResponse> {
    let has_permission = arc
        .pool
        .user_has_permission(user.sub, &UserPermission::DeleteTorrent)
        .await?;

    if !has_permission {
        let torrent = arc.pool.find_torrent(form.id).await?;

        if torrent.created_by_id != user.sub {
            arc.pool
                .require_permission(user.sub, &UserPermission::DeleteTorrent, req.path())
                .await?;
        }

        let hours_since_upload = (Local::now() - torrent.created_at).num_hours();
        if hours_since_upload >= 24 {
            return Err(Error::TorrentDeletionWindowExpired);
        }
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

    let torrent_id = form.id;

    form.displayed_reason = Some(displayed_reason);
    arc.pool.remove_torrent(&form, user.sub).await?;

    let mut url = arc.env.tracker.url_internal.clone();
    url.path_segments_mut()
        .unwrap()
        .push("api")
        .push("torrents")
        .push(&torrent_id.to_string());

    let res = arc
        .internal_http_client
        .delete(url)
        .header("x-api-key", arc.env.tracker.api_key.clone())
        .send()
        .await;

    log::warn!(
        "Tried to mark torrent as deleted in tracker and got: {:?}",
        res
    );

    Ok(HttpResponse::Ok().json(json!({"result": "success"})))
}
