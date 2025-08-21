use sqlx::PgPool;
use arcadia_common::error::{Error, Result};
use crate::{
    models::{
        title_group_comment::{TitleGroupComment, UserCreatedTitleGroupComment},
        user::User,
    },
};

pub async fn create_title_group_comment(
    pool: &PgPool,
    title_group_comment: &UserCreatedTitleGroupComment,
    current_user: &User,
) -> Result<TitleGroupComment> {
    let created_title_group_comment = sqlx::query_as!(
        TitleGroupComment,
        r#"
            INSERT INTO title_group_comments (content, title_group_id, created_by_id,
                                              refers_to_torrent_id, answers_to_comment_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
        "#,
        title_group_comment.content,
        title_group_comment.title_group_id,
        current_user.id,
        title_group_comment.refers_to_torrent_id,
        title_group_comment.answers_to_comment_id
    )
    .fetch_one(pool)
    .await
    .map_err(Error::CouldNotCreateTitleGroupComment)?;

    Ok(created_title_group_comment)
}
