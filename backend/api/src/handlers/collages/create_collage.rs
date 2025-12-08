use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::collage::{Collage, UserCreatedCollage},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create collage",
    tag = "Collages",
    path = "/api/collages",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully created the collage", body=Collage),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    collage: Json<UserCreatedCollage>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let collage = arc.pool.create_collage(&collage, user.sub).await?;

    Ok(HttpResponse::Created().json(collage))
}
