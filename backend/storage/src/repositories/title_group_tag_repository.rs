use crate::{
    connection_pool::ConnectionPool,
    models::{
        common::PaginatedResults,
        title_group_tag::{
            EditedTitleGroupTag, SearchTitleGroupTagsQuery, TitleGroupTag, TitleGroupTagEnriched,
            TitleGroupTagLite, UserCreatedTitleGroupTag,
        },
        user::UserLite,
    },
};
use arcadia_common::error::{Error, Result};
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

    pub async fn update_title_group_tag(
        &self,
        edited_tag: &EditedTitleGroupTag,
    ) -> Result<TitleGroupTag> {
        let sanitized_name = Self::sanitize_tag_name(&edited_tag.name);

        let updated_tag = sqlx::query_as!(
            TitleGroupTag,
            r#"
            UPDATE title_group_tags
            SET name = $1, synonyms = $2
            WHERE id = $3
            RETURNING
                id,
                name,
                synonyms as "synonyms!: Vec<String>",
                created_at,
                created_by_id
            "#,
            sanitized_name,
            &edited_tag.synonyms as _,
            edited_tag.id
        )
        .fetch_optional(self.borrow())
        .await
        .map_err(Error::CouldNotUpdateTitleGroupTag)?
        .ok_or(Error::TitleGroupTagNotFound)?;

        Ok(updated_tag)
    }

    pub async fn delete_title_group_tag(&self, tag_id: i32) -> Result<()> {
        let rows_affected = sqlx::query!(
            r#"
            DELETE FROM title_group_tags
            WHERE id = $1
            "#,
            tag_id
        )
        .execute(self.borrow())
        .await
        .map_err(Error::CouldNotDeleteTitleGroupTag)?;

        if rows_affected.rows_affected() == 0 {
            return Err(Error::TitleGroupTagNotFound);
        }

        Ok(())
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

    pub async fn search_title_group_tags_lite(
        &self,
        query: &str,
        page: u32,
        page_size: u32,
    ) -> Result<PaginatedResults<TitleGroupTagLite>> {
        let offset = ((page - 1) * page_size) as i64;
        let limit = page_size as i64;

        let total_items = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)::BIGINT
            FROM title_group_tags
            WHERE
                name ILIKE '%' || $1 || '%'
                OR EXISTS (
                    SELECT 1
                    FROM unnest(synonyms) AS synonym
                    WHERE synonym ILIKE '%' || $1 || '%'
                )
            "#,
            query
        )
        .fetch_one(self.borrow())
        .await?
        .unwrap_or(0);

        let results = sqlx::query_as!(
            TitleGroupTagLite,
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
            LIMIT $2 OFFSET $3
            "#,
            query,
            limit,
            offset
        )
        .fetch_all(self.borrow())
        .await?;

        Ok(PaginatedResults {
            results,
            page,
            page_size,
            total_items,
        })
    }

    pub async fn search_title_group_tags(
        &self,
        query: &SearchTitleGroupTagsQuery,
    ) -> Result<PaginatedResults<TitleGroupTagEnriched>> {
        let offset = ((query.page - 1) * query.page_size) as i64;
        let limit = query.page_size as i64;

        let total_items = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*)::BIGINT
            FROM title_group_tags t
            WHERE
                t.name ILIKE '%' || $1 || '%'
                OR EXISTS (
                    SELECT 1
                    FROM unnest(t.synonyms) AS synonym
                    WHERE synonym ILIKE '%' || $1 || '%'
                )
            "#,
            query.name
        )
        .fetch_one(self.borrow())
        .await?
        .unwrap_or(0);

        let results = sqlx::query_as!(
            TitleGroupTagEnriched,
            r#"
            SELECT
                t.id,
                t.name,
                t.synonyms AS "synonyms!: Vec<String>",
                t.created_at,
                ROW(u.id, u.username, u.warned, u.banned) AS "created_by!: UserLite",
                COALESCE((
                    SELECT COUNT(*)::INT
                    FROM title_group_applied_tags at
                    WHERE at.tag_id = t.id
                ), 0) AS "uses!"
            FROM title_group_tags t
            JOIN users u ON t.created_by_id = u.id
            WHERE
                t.name ILIKE '%' || $1 || '%'
                OR EXISTS (
                    SELECT 1
                    FROM unnest(t.synonyms) AS synonym
                    WHERE synonym ILIKE '%' || $1 || '%'
                )
            ORDER BY
                CASE WHEN $2 = 'name' AND $3 = 'asc' THEN t.name END ASC,
                CASE WHEN $2 = 'name' AND $3 = 'desc' THEN t.name END DESC,
                CASE WHEN $2 = 'created_at' AND $3 = 'asc' THEN t.created_at END ASC,
                CASE WHEN $2 = 'created_at' AND $3 = 'desc' THEN t.created_at END DESC,
                CASE WHEN $2 = 'uses' AND $3 = 'asc' THEN (
                    SELECT COUNT(*)::INT
                    FROM title_group_applied_tags at
                    WHERE at.tag_id = t.id
                ) END ASC,
                CASE WHEN $2 = 'uses' AND $3 = 'desc' THEN (
                    SELECT COUNT(*)::INT
                    FROM title_group_applied_tags at
                    WHERE at.tag_id = t.id
                ) END DESC,
                t.name
            LIMIT $4 OFFSET $5
            "#,
            query.name,
            query.order_by_column.to_string(),
            query.order_by_direction.to_string(),
            limit,
            offset
        )
        .fetch_all(self.borrow())
        .await?;

        Ok(PaginatedResults {
            results,
            page: query.page,
            page_size: query.page_size,
            total_items,
        })
    }
}
