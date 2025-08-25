use crate::{
    connection_pool::ConnectionPool,
    models::wiki::{UserCreatedWikiArticle, WikiArticle},
};
use arcadia_common::error::{Error, Result};
use serde_json::Value;
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_wiki_article(
        &self,
        article: &UserCreatedWikiArticle,
        current_user_id: i64,
    ) -> Result<WikiArticle> {
        let created_article = sqlx::query_as!(
            WikiArticle,
            r#"
                INSERT INTO wiki_articles (title, body, created_by_id, updated_by_id)
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
            article.title,
            article.body,
            current_user_id,
            current_user_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateWikiArticle)?;

        Ok(created_article)
    }

    pub async fn find_wiki_article(&self, article_id: i64) -> Result<Value> {
        let article = sqlx::query!(
            r#"
            SELECT
                json_build_object(
                    'id', wa.id,
                    'title', wa.title,
                    'created_at', wa.created_at,
                    'created_by', json_build_object(
                        'id', cb.id,
                        'username', cb.username,
                        'warned', cb.warned,
                        'banned', cb.banned
                    ),
                    'updated_at', wa.updated_at,
                    'updated_by', json_build_object(
                        'id', ub.id,
                        'username', ub.username,
                        'warned', ub.warned,
                        'banned', ub.banned
                    ),
                    'body', wa.body
                ) AS article_json
            FROM
                wiki_articles wa
            JOIN
                users cb ON wa.created_by_id = cb.id
            JOIN
                users ub ON wa.updated_by_id = ub.id
            WHERE
                wa.id = $1
            "#,
            article_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFindWikiArticle)?;

        Ok(article.article_json.unwrap())
    }
}
