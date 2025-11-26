use crate::Arcadia;
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::title_group_tag::TitleGroupTagSearchResult, redis::RedisPoolInterface,
};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct SearchTagsQuery {
    pub name: String,
}

#[utoipa::path(
    get,
    operation_id = "Search title group tags",
    tag = "Search",
    path = "/api/search/title-group-tags",
    params(
        ("name" = String, Query, description = "Search query (searches in tag name and synonyms)")
    ),
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "List of matching tags with their names and synonyms", body=Vec<TitleGroupTagSearchResult>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<SearchTagsQuery>,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    let results = arc.pool.search_title_group_tags(&query.name).await?;

    Ok(HttpResponse::Ok().json(results))
}
