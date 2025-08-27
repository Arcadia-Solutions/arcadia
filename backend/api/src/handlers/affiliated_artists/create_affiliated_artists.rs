use crate::{handlers::UserId, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::artist::{AffiliatedArtistHierarchy, UserCreatedAffiliatedArtist};

#[utoipa::path(
    post,
    operation_id = "Create artist affiliation",
    tag = "Affiliated Artist",
    path = "/api/affiliated-artists",
    responses(
        (status = 200, description = "Successfully created the artist affiliations", body=Vec<AffiliatedArtistHierarchy>),
    )
)]
pub async fn exec(
    artists: web::Json<Vec<UserCreatedAffiliatedArtist>>,
    arc: web::Data<Arcadia>,
    current_user_id: UserId,
) -> Result<HttpResponse> {
    let affiliations = arc
        .pool
        .create_artists_affiliation(&artists, current_user_id.0)
        .await?;

    Ok(HttpResponse::Created().json(affiliations))
}
