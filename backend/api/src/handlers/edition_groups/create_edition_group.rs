use crate::{handlers::UserId, Arcadia};
use actix_web::{
    web::{self, Data, Json},
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
    current_user_id: UserId,
) -> Result<HttpResponse> {
    let edition_group = arc
        .pool
        .create_edition_group(&form, current_user_id.0)
        .await?;

    Ok(HttpResponse::Created().json(edition_group))
}
