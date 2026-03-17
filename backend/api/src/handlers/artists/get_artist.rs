use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{models::artist::Artist, redis::RedisPoolInterface};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetArtistQuery {
    id: i64,
}

#[utoipa::path(
    get,
    operation_id = "Get artist",
    tag = "Artist",
    path = "/api/artists",
    params (GetArtistQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully got the artist", body=Artist),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<GetArtistQuery>,
    arc: Data<Arcadia<R>>,
    _user: Authdata,
) -> Result<HttpResponse> {
    let artist = arc.pool.find_artist_by_id(query.id).await?;

    Ok(HttpResponse::Ok().json(artist))
}
