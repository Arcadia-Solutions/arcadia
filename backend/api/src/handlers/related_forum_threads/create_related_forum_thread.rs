use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{forum::CreateRelatedForumThread, user::UserPermission},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create related forum thread",
    tag = "Related Forum Threads",
    path = "/api/related-forum-threads",
    security(
        ("http" = ["Bearer"])
    ),
    request_body = CreateRelatedForumThread,
    responses(
        (status = 200, description = "Successfully created the related forum thread"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    payload: Json<CreateRelatedForumThread>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
    req: HttpRequest,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(
            user.sub,
            &UserPermission::ManageRelatedForumThread,
            req.path(),
        )
        .await?;

    arc.pool
        .create_related_forum_thread(&payload, user.sub)
        .await?;

    Ok(HttpResponse::Ok().finish())
}
