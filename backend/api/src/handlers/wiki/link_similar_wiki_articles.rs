use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse,
};
use arcadia_common::error::Result;
use arcadia_storage::{
    models::{user::UserPermission, wiki::SimilarWikiArticlesLink},
    redis::RedisPoolInterface,
};
use serde_json::json;

#[utoipa::path(
    post,
    operation_id = "Link similar wiki articles",
    tag = "Wiki",
    path = "/api/wiki/articles/similar",
    request_body = SimilarWikiArticlesLink,
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully linked the wiki articles"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    link: Json<SimilarWikiArticlesLink>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
    req: HttpRequest,
) -> Result<HttpResponse> {
    arc.pool
        .require_permission(
            user.sub,
            &UserPermission::LinkSimilarWikiArticles,
            req.path(),
        )
        .await?;

    arc.pool.link_similar_wiki_articles(&link, user.sub).await?;

    Ok(HttpResponse::Ok().json(json!({"result": "success"})))
}
