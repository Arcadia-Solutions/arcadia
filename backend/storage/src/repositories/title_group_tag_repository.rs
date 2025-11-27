use crate::{
    connection_pool::ConnectionPool,
    models::title_group_tag::{TitleGroupTag, TitleGroupTagSearchResult, UserCreatedTitleGroupTag},
};
use arcadia_common::error::{Error, Result};
use sqlx::PgPool;
use std::borrow::Borrow;

impl ConnectionPool {
    fn sanitize_tag_name(name: &str) -> String {
        name.trim()
            .to_lowercase()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(".")
    }

    pub async fn create_title_group_tag(
        &self,
        tag: &UserCreatedTitleGroupTag,
        user_id: i32,
    ) -> Result<TitleGroupTag> {
        let sanitized_name = Self::sanitize_tag_name(&tag.name);

        let mut created_tag = sqlx::query_as!(
            TitleGroupTag,
            r#"
            INSERT INTO title_group_tags (name, created_by_id)
            VALUES ($1, $2)
            ON CONFLICT (name) DO NOTHING
            RETURNING
                id,
                name,
                synonyms as "synonyms!: Vec<String>",
                created_at,
                created_by_id
            "#,
            sanitized_name,
            user_id
        )
        .fetch_one(self.borrow())
        .await;

        // the tag already exists
        if created_tag.is_err() {
            created_tag = sqlx::query_as!(
                TitleGroupTag,
                r#"
                SELECT
                    id,
                    name,
                    synonyms as "synonyms!: Vec<String>",
                    created_at,
                    created_by_id
                FROM title_group_tags
                WHERE name = $1
                "#,
                sanitized_name
            )
            .fetch_one(self.borrow())
            .await;
        }

        created_tag.map_err(Error::CouldNotCreateTitleGroupTag)
    }

    async fn find_tag_id_by_name(&self, tag_name: &str) -> Result<Option<i32>> {
        let sanitized_name = Self::sanitize_tag_name(tag_name);

        let tag_id = sqlx::query_scalar!(
            r#"
            SELECT id FROM title_group_tags WHERE name = $1
            "#,
            sanitized_name
        )
        .fetch_optional(self.borrow())
        .await?;

        Ok(tag_id)
    }

    pub async fn apply_tag_to_title_group(
        &self,
        title_group_id: i32,
        tag_id: i32,
        user_id: i32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO title_group_applied_tags (title_group_id, tag_id, created_by_id)
            VALUES ($1, $2, $3)
            ON CONFLICT DO NOTHING
            "#,
            title_group_id,
            tag_id,
            user_id
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    pub async fn remove_tag_from_title_group(
        &self,
        title_group_id: i32,
        tag_name: &str,
    ) -> Result<()> {
        let tag_id = self.find_tag_id_by_name(tag_name).await?;

        let tag_id =
            tag_id.ok_or_else(|| Error::BadRequest(format!("Tag '{}' not found", tag_name)))?;

        sqlx::query!(
            r#"
            DELETE FROM title_group_applied_tags
            WHERE title_group_id = $1 AND tag_id = $2
            "#,
            title_group_id,
            tag_id
        )
        .execute(self.borrow())
        .await?;

        Ok(())
    }

    pub async fn search_title_group_tags(
        &self,
        query: &str,
    ) -> Result<Vec<TitleGroupTagSearchResult>> {
        let search_pattern = format!("%{}%", query);

        let results = sqlx::query_as!(
            TitleGroupTagSearchResult,
            r#"
            SELECT
                name,
                synonyms as "synonyms!: Vec<String>",
                id
            FROM title_group_tags
            WHERE
                name ILIKE '%' || $1 || '%'
                OR EXISTS (
                    SELECT 1
                    FROM unnest(synonyms) AS synonym
                    WHERE synonym ILIKE '%' || $1 || '%'
                )
            ORDER BY name
            LIMIT 10
            "#,
            search_pattern
        )
        .fetch_all(<ConnectionPool as Borrow<PgPool>>::borrow(self))
        .await?;

        Ok(results)
    }
}
