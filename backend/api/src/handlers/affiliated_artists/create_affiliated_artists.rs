use crate::{handlers::UserId, Arcadia};
use actix_web::{
    web::{self, Data, Json},
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
    current_user_id: UserId,
) -> Result<HttpResponse> {
    let affiliations = arc
        .pool
        .create_artists_affiliation(&artists, current_user_id.0)
        .await?;

    Ok(HttpResponse::Created().json(affiliations))
}
