use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::master_group::{MasterGroup, UserCreatedMasterGroup};

#[utoipa::path(
    post,
    operation_id = "Create master group",
    tag = "Master Group",
    path = "/api/master-groups",
    responses(
        (status = 200, description = "Successfully created the master group", body=MasterGroup),
    )
)]
pub async fn exec(
    form: web::Json<UserCreatedMasterGroup>,
    arc: web::Data<Arcadia>,
    user: Authdata,
) -> Result<HttpResponse> {
    let master_group = arc.pool.create_master_group(&form, user.sub).await?;

    Ok(HttpResponse::Created().json(master_group))
}
