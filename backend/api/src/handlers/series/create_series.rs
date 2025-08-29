use crate::{handlers::User, Arcadia};
use actix_web::{
    web::{self, Data, Json},
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
    current_user: User,
) -> Result<HttpResponse> {
    let series = arc.pool.create_series(&serie, &current_user).await?;

    Ok(HttpResponse::Created().json(series))
}
