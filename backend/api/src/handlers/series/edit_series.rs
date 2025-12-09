use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{
        series::{EditedSeries, Series},
        user::UserClass,
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    put,
    operation_id = "Edit series",
    tag = "Series",
    path = "/api/series",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully edited the series", body=Series),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    form: Json<EditedSeries>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let series = arc.pool.find_series(&form.id).await?;

    if user.class != UserClass::Staff && series.created_by_id != user.sub {
        return Err(Error::InsufficientPrivileges);
    }

    let updated_series = arc.pool.update_series(&form).await?;

    Ok(HttpResponse::Ok().json(updated_series))
}
