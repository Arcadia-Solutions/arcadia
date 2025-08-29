use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{web::Data, HttpResponse};
use actix_web_lab::extract::Query;
use arcadia_common::error::Result;
use arcadia_storage::redis::RedisPoolInterface;
use serde::Deserialize;
use serde_json::json;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct RemoveAffiliatedArtistsQuery {
    affiliation_ids: Vec<i64>,
}

#[utoipa::path(
    delete,
    operation_id = "Delete artist affiliation",
    tag = "Affiliated Artist",
    path = "/api/affiliated-artists",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully removed the artist affiliations"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<RemoveAffiliatedArtistsQuery>,
    arc: Data<Arcadia<R>>,
    _: Authdata,
) -> Result<HttpResponse> {
    // TODO: add protection based on user class
    arc.pool
        .delete_artists_affiliation(&query.affiliation_ids)
        .await?;

    Ok(HttpResponse::Ok().json(json!({"result": "success"})))
}
