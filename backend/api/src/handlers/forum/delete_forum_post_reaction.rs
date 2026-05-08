use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Path},
    HttpRequest, HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{forum::ForumPostHierarchy, user::UserPermission},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    delete,
    operation_id = "Delete forum post reaction",
    tag = "Forum",
    path = "/api/forum/post/{id}/reaction",
    security(
      ("http" = ["Bearer"])
    ),
    params(
        ("id" = i64, Path, description = "Forum post id")
    ),
    responses(
        (status = 200, description = "Successfully delete the forum post reaction", body = ForumPostHierarchy),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    post_id: Path<i64>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
    req: HttpRequest,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(user.sub, &UserPermission::CreateForumPost, req.path())
        .await?;

    let forum_post = arc
        .pool
        .delete_forum_post_reaction_and_get_post(*post_id, user.sub)
        .await?;

    Ok(HttpResponse::Ok().json(forum_post))
}
