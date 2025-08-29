use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::title_group_comment::{TitleGroupComment, UserCreatedTitleGroupComment},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create title group comment",
    tag = "Title Group",
    path = "/api/title-groups/comments",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully posted the comment", body=TitleGroupComment),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    comment: Json<UserCreatedTitleGroupComment>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let title_group_comment = arc
        .pool
        .create_title_group_comment(&comment, user.sub)
        .await?;

    Ok(HttpResponse::Created().json(title_group_comment))
}
