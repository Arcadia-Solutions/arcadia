use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::{handlers::User, Arcadia};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::torrent::TorrentToDelete;

#[utoipa::path(
    delete,
    operation_id = "Delete torrent",
    tag = "Torrent",
    path = "/api/torrents",
    responses(
        (status = 200, description = "Torrent deleted"),
    )
)]
pub async fn exec(
    mut form: web::Json<TorrentToDelete>,
    arc: web::Data<Arcadia>,
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
