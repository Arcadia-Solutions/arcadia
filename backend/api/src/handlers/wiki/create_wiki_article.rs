use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::wiki::{UserCreatedWikiArticle, WikiArticle};

#[utoipa::path(
    post,
    operation_id = "Create wiki article",
    tag = "Wiki",
    path = "/api/wiki/articles",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully created the wiki article", body=WikiArticle),
    )
)]
pub async fn exec(
    article: web::Json<UserCreatedWikiArticle>,
    arc: web::Data<Arcadia>,
    user: Authdata,
) -> Result<HttpResponse> {
    if user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }

    let article = arc.pool.create_wiki_article(&article, user.sub).await?;

    Ok(HttpResponse::Created().json(article))
}
