use crate::Arcadia;
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{models::collage::GetCollageEntriesQuery, redis::RedisPoolInterface};

#[utoipa::path(
    get,
    operation_id = "Get collage entries",
    tag = "Collages",
    path = "/api/collages/entries",
    params(GetCollageEntriesQuery),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Paginated collage entries"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<GetCollageEntriesQuery>,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    let entries = arc
        .pool
        .find_collage_entries(query.collage_id, query.page, query.page_size)
        .await?;

    Ok(HttpResponse::Ok().json(entries))
}
