use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::series::{Series, UserCreatedSeries},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create series",
    tag = "Series",
    path = "/api/series",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully created the series", body=Series),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    serie: Json<UserCreatedSeries>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let series = arc.pool.create_series(&serie, user.sub).await?;

    Ok(HttpResponse::Created().json(series))
}
