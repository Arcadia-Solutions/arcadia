use crate::{handlers::UserId, Arcadia};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::master_group::{MasterGroup, UserCreatedMasterGroup},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create master group",
    tag = "Master Group",
    path = "/api/master-groups",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully created the master group", body=MasterGroup),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    form: Json<UserCreatedMasterGroup>,
    arc: Data<Arcadia<R>>,
    current_user_id: UserId,
) -> Result<HttpResponse> {
    let master_group = arc
        .pool
        .create_master_group(&form, current_user_id.0)
        .await?;

    Ok(HttpResponse::Created().json(master_group))
}
