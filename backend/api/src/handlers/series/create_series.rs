use crate::{middlewares::jwt_middleware::JwtAuthData, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::series::{Series, UserCreatedSeries};

#[utoipa::path(
    post,
    operation_id = "Create series",
    tag = "Series",
    path = "/api/series",
    responses(
        (status = 200, description = "Successfully created the series", body=Series),
    )
)]
pub async fn exec(
    serie: web::Json<UserCreatedSeries>,
    arc: web::Data<Arcadia>,
    user: JwtAuthData,
) -> Result<HttpResponse> {
    let series = arc.pool.create_series(&serie, user.sub).await?;

    Ok(HttpResponse::Created().json(series))
}
