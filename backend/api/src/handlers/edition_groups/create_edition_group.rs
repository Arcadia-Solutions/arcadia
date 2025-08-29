use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::edition_group::{EditionGroup, UserCreatedEditionGroup},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create edition group",
    tag = "Edition Group",
    path = "/api/edition-groups",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully created the edition_group", body=EditionGroup),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    form: Json<UserCreatedEditionGroup>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let edition_group = arc.pool.create_edition_group(&form, user.sub).await?;

    Ok(HttpResponse::Created().json(edition_group))
}
