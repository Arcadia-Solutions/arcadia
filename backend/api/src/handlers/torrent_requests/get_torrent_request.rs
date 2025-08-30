use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::torrent_request::TorrentRequestAndAssociatedData;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetTorrentRequestQuery {
    id: i64,
}

#[utoipa::path(
    get,
    operation_id = "Get torrent requests",
    tag = "Torrent Request",
    path = "/api/torrent-requests",
    params(GetTorrentRequestQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully got the torrent request with associated data", body=TorrentRequestAndAssociatedData),
    )
)]
pub async fn exec(
    arc: web::Data<Arcadia>,
    query: web::Query<GetTorrentRequestQuery>,
    _: Authdata,
) -> Result<HttpResponse> {
    let torrent_request = arc.pool.find_torrent_request_hierarchy(query.id).await?;

    Ok(HttpResponse::Ok().json(torrent_request))
}
