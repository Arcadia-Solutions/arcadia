use crate::{handlers::User, Arcadia};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{models::user::EditedUser, redis::RedisPoolInterface};
use serde_json::json;

#[utoipa::path(
    put,
    operation_id = "Edit user",
    tag = "User",
    path = "/api/users",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully edited the user"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    form: Json<EditedUser>,
    current_user: User,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    arc.pool.update_user(current_user.id, &form).await?;

    Ok(HttpResponse::Ok().json(json!({"status": "success"})))
}
