use crate::Arcadia;
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::wiki::WikiArticle;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetWikiArticleQuery {
    id: i64,
}

#[utoipa::path(
    get,
    operation_id = "Get wiki article",
    tag = "Wiki",
    path = "/api/wiki/articles",
    params(GetWikiArticleQuery),
    responses(
        (status = 200, description = "Successfully found the wiki article", body=WikiArticle),
    )
)]
pub async fn exec(
    query: web::Query<GetWikiArticleQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let article = arc.pool.find_wiki_article(query.id).await?;

    Ok(HttpResponse::Ok().json(article))
}
