use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{
        user::UserPermission,
        wiki::{EditedWikiArticle, WikiArticle},
    },
    redis::RedisPoolInterface,
};

#[utoipa::path(
    put,
    operation_id = "Edit wiki article",
    tag = "Wiki",
    path = "/api/wiki/articles",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully created the wiki article", body=WikiArticle),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    article: Json<EditedWikiArticle>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if !arc
        .pool
        .user_has_permission(user.sub, &UserPermission::EditWikiArticle)
        .await?
    {
        return Err(Error::InsufficientPrivileges);
    }

    let article = arc.pool.edit_wiki_article(&article, user.sub).await?;

    Ok(HttpResponse::Created().json(article))
}
