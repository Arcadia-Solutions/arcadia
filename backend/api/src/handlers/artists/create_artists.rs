use crate::{handlers::UserId, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::artist::{Artist, UserCreatedArtist};

#[utoipa::path(
    post,
    operation_id = "Create artist",
    tag = "Artist",
    path = "/api/artists",
    responses(
        (status = 201, description = "Successfully created the artists, returned in the same order as the one sent.
            In the case of a db conflict (duplicate), the existing entry is returned (can be seen with the created_at attribute).", body=Vec<Artist>),
    )
)]
pub async fn exec(
    artists: web::Json<Vec<UserCreatedArtist>>,
    arc: web::Data<Arcadia>,
    current_user_id: UserId,
) -> Result<HttpResponse> {
    let artists = arc.pool.create_artists(&artists, current_user_id.0).await?;

    Ok(HttpResponse::Created().json(artists))
}
