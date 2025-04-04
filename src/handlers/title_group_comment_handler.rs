use crate::{
    Arcadia, Result,
    models::{title_group_comment::UserCreatedTitleGroupComment, user::User},
    repositories::title_group_comment_repository::create_title_group_comment,
};
use actix_web::{HttpResponse, web};

pub async fn add_title_group_comment(
    comment: web::Json<UserCreatedTitleGroupComment>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let title_group_comment =
        create_title_group_comment(&arc.pool, &comment, &current_user).await?;

    Ok(HttpResponse::Created().json(title_group_comment))
}
