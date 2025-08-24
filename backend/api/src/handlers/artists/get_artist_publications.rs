use crate::Arcadia;
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::artist::ArtistAndTitleGroupsLite;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetArtistPublicationsQuery {
    id: i64,
}

#[utoipa::path(
    get,
    path = "/api/artists",
    params (GetArtistPublicationsQuery),
    responses(
        (status = 200, description = "Successfully got the artist's pulications", body=ArtistAndTitleGroupsLite),
    )
)]
pub async fn exec(
    query: web::Query<GetArtistPublicationsQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let artist_publication = arc.pool.find_artist_publications(&query.id).await?;

    Ok(HttpResponse::Ok().json(artist_publication))
}
