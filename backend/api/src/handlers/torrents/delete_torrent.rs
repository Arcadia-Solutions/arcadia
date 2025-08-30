use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::torrent::TorrentToDelete;

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
pub async fn exec(
    mut form: web::Json<TorrentToDelete>,
    arc: web::Data<Arcadia>,
    user: Authdata,
) -> Result<HttpResponse> {
    if user.class != "staff" {
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
