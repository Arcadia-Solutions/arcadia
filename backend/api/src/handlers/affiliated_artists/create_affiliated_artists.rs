use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::artist::{AffiliatedArtistHierarchy, UserCreatedAffiliatedArtist},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create artist affiliation",
    tag = "Affiliated Artist",
    path = "/api/affiliated-artists",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully created the artist affiliations", body=Vec<AffiliatedArtistHierarchy>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    artists: Json<Vec<UserCreatedAffiliatedArtist>>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let affiliations = arc
        .pool
        .create_artists_affiliation(&artists, user.sub)
        .await?;

    Ok(HttpResponse::Created().json(affiliations))
}
