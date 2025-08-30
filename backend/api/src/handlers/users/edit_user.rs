use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::user::EditedUser;
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
pub async fn exec(
    form: web::Json<EditedUser>,
    arc: web::Data<Arcadia>,
    user: Authdata,
) -> Result<HttpResponse> {
    arc.pool.update_user(user.sub, &form).await?;

    Ok(HttpResponse::Ok().json(json!({"status": "success"})))
}
