use crate::Arcadia;
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
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
    responses(
        (status = 200, description = "Successfully removed the artist affiliations"),
    )
)]
pub async fn exec(
    query: actix_web_lab::extract::Query<RemoveAffiliatedArtistsQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    // TODO: add protection based on user class
    arc.pool
        .delete_artists_affiliation(&query.affiliation_ids)
        .await?;

    Ok(HttpResponse::Ok().json(json!({"result": "success"})))
}
