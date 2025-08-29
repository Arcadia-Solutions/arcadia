use crate::Arcadia;
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{models::artist::ArtistAndTitleGroupsLite, redis::RedisPoolInterface};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetArtistPublicationsQuery {
    id: i64,
}

#[utoipa::path(
    get,
    operation_id = "Get artist publications",
    tag = "Artist",
    path = "/api/artists",
    params (GetArtistPublicationsQuery),
    responses(
        (status = 200, description = "Successfully got the artist's publications", body=ArtistAndTitleGroupsLite),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<GetArtistPublicationsQuery>,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    let artist_publication = arc.pool.find_artist_publications(&query.id).await?;

    Ok(HttpResponse::Ok().json(artist_publication))
}
