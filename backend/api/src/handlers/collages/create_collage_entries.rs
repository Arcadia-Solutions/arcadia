use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::collage::{CollageEntry, UserCreatedCollageEntry},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Inserts entries into a collage",
    tag = "Collages",
    path = "/api/collages/entries",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully created the collage entries", body=Vec<CollageEntry>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    collage_entries: Json<Vec<UserCreatedCollageEntry>>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    // TODO: instead, we should return Vec<CollageAndAssociatedData> so the frontend can hydrate the page
    // without having to refetch everything
    let collage_entries = arc
        .pool
        .create_collage_entries(&collage_entries, user.sub)
        .await?;

    Ok(HttpResponse::Created().json(collage_entries))
}
