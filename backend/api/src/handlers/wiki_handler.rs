use crate::{handlers::User, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::wiki::{UserCreatedWikiArticle, WikiArticle};
use serde::Deserialize;
use utoipa::IntoParams;

#[utoipa::path(
    post,
    path = "/api/wiki/article",
    responses(
        (status = 200, description = "Successfully created the wiki article", body=WikiArticle),
    )
)]
pub async fn add_wiki_article(
    article: web::Json<UserCreatedWikiArticle>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    if current_user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }

    let article = arc
        .pool
        .create_wiki_article(&article, current_user.id)
        .await?;

    Ok(HttpResponse::Created().json(article))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetWikiArticleQuery {
    id: i64,
}

#[utoipa::path(
    get,
    path = "/api/wiki/article",
    params(GetWikiArticleQuery),
    responses(
        (status = 200, description = "Successfully found the wiki article", body=WikiArticle),
    )
)]
pub async fn get_wiki_article(
    query: web::Query<GetWikiArticleQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let article = arc.pool.find_wiki_article(query.id).await?;

    Ok(HttpResponse::Ok().json(article))
}
