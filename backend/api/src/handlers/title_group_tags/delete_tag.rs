use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{models::user::UserPermission, redis::RedisPoolInterface};
use serde::Deserialize;
use serde_json::json;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteTagRequest {
    pub id: i32,
}

#[utoipa::path(
    delete,
    operation_id = "Delete title group tag",
    tag = "Title Group Tag",
    path = "/api/title-group-tags",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully deleted the title group tag"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    request: Json<DeleteTagRequest>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::DeleteTitleGroupTag)
        .await?
    {
        return Err(Error::InsufficientPermissions(format!(
            "{:?}",
            UserPermission::DeleteTitleGroupTag
        )));
    }

    arc.pool.delete_title_group_tag(request.id).await?;

    Ok(HttpResponse::Ok().json(json!({"result": "success"})))
}
