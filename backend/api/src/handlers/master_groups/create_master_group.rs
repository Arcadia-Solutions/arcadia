use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
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
    user: Authdata,
) -> Result<HttpResponse> {
    let master_group = arc.pool.create_master_group(&form, user.sub).await?;

    Ok(HttpResponse::Created().json(master_group))
}
