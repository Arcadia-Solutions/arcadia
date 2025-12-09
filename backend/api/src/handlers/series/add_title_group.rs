use crate::Arcadia;
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{models::title_group::TitleGroup, redis::RedisPoolInterface};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddTitleGroupToSeriesRequest {
    pub series_id: i64,
    pub title_group_id: i32,
}

#[utoipa::path(
    post,
    operation_id = "Add title group to series",
    tag = "Series",
    path = "/api/series/title-group",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully attached the title group to the series", body=TitleGroup),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    form: Json<AddTitleGroupToSeriesRequest>,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    arc.pool
        .assign_title_group_to_series(form.title_group_id, form.series_id)
        .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"result": "success"})))
}
