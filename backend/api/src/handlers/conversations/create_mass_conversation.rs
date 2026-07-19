use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{
        conversation::{MassMessageRequest, MassMessageResult},
        user::UserPermission,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create mass conversation",
    tag = "Conversation",
    path = "/api/conversations/mass",
    security(
      ("http" = ["Bearer"])
    ),
    description = "Sends a private message to every user matching the registration filter (username and/or registration date range), across all pages.",
    responses(
        (status = 200, description = "Successfully sent the message to every matching user", body=MassMessageResult),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    payload: Json<MassMessageRequest>,
    current_user: Authdata,
    arc: Data<Arcadia<R>>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(current_user.sub, &UserPermission::SendMassPm, req.path())
        .await?;

    let result = arc
        .pool
        .create_mass_conversation(&payload, current_user.sub, &arc.notification_sender)
        .await?;

    Ok(HttpResponse::Ok().json(result))
}
