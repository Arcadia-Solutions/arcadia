use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{forum::ForumSubCategoryAllowedPoster, user::UserPermission},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    delete,
    operation_id = "Remove forum sub-category allowed poster",
    tag = "Forum",
    path = "/api/forum/sub-category/allowed-poster",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully removed the allowed poster"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    allowed_poster: Json<ForumSubCategoryAllowedPoster>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
    req: HttpRequest,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(user.sub, &UserPermission::EditForumSubCategory, req.path())
        .await?;

    arc.pool
        .remove_forum_sub_category_allowed_poster(
            allowed_poster.forum_sub_category_id,
            allowed_poster.user_id,
        )
        .await?;

    Ok(HttpResponse::Ok().finish())
}
