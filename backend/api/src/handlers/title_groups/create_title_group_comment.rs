use crate::{handlers::User, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::models::title_group_comment::{
    TitleGroupComment, UserCreatedTitleGroupComment,
};

#[utoipa::path(
    post,
    operation_id = "Create title group comment",
    tag = "Title Group",
    path = "/api/title-groups/comments",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully posted the comment", body=TitleGroupComment),
    )
)]
pub async fn exec(
    comment: web::Json<UserCreatedTitleGroupComment>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let title_group_comment = arc
        .pool
        .create_title_group_comment(&comment, &current_user)
        .await?;

    Ok(HttpResponse::Created().json(title_group_comment))
}
