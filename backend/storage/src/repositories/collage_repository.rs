use crate::{
    connection_pool::ConnectionPool,
    models::{
        collage::{
            Collage, CollageCategory, CollageEntry, CollageLite, CollageSearchResult,
            SearchCollagesLiteQuery, SearchCollagesQuery, UserCreatedCollage,
            UserCreatedCollageEntry,
        },
        common::PaginatedResults,
    },
};
use arcadia_common::error::{Error, Result};
use sqlx::{query_as_unchecked, query_scalar};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_collage(
        &self,
        collage: &UserCreatedCollage,
        user_id: i32,
    ) -> Result<Collage> {
        let created_collage = sqlx::query_as!(
            Collage,
            r#"
                INSERT INTO collage (created_by_id, name, cover, description, tags, category)
                VALUES ($1, $2, $3, $4, $5, $6::collage_category_enum)
                RETURNING id, created_at, created_by_id, name, cover, description, tags,
                category as "category: CollageCategory"
            "#,
            user_id,
            collage.name,
            collage.cover,
            collage.description,
            &collage.tags,
            collage.category as _,
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateCollage)?;

        Ok(created_collage)
    }

    pub async fn create_collage_entries(
        &self,
        collage_entries: &[UserCreatedCollageEntry],
        user_id: i32,
    ) -> Result<Vec<CollageEntry>> {
        let mut created_entries = Vec::with_capacity(collage_entries.len());

        // TODO: do it as a transaction
        for entry in collage_entries {
            let created = sqlx::query_as!(
                CollageEntry,
                r#"
                    INSERT INTO collage_entry (
                        created_by_id,
                        title_group_id,
                        collage_id,
                        note
                    )
                    VALUES ($1, $2, $3, $4)
                    RETURNING *
                "#,
                user_id,
                entry.title_group_id,
                entry.collage_id,
                entry.note
            )
            .fetch_one(self.borrow())
            .await
            .map_err(|e| Error::CouldNotCreateCollageEntry(e.to_string()))?;

            created_entries.push(created);
        }

        Ok(created_entries)
    }

    pub async fn find_collage(&self, collage_id: &i64) -> Result<Collage> {
        let collage = sqlx::query_as!(
            Collage,
            r#"
            SELECT id, created_at, created_by_id, name, cover, description, tags,
                   category as "category: CollageCategory"
            FROM collage
            WHERE id = $1
            "#,
            collage_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotFetchCollage)?;

        Ok(collage)
    }

    pub async fn search_collages(
        &self,
        form: &SearchCollagesQuery,
    ) -> Result<PaginatedResults<CollageSearchResult>> {
        let offset = (form.page - 1) * form.page_size;

        let total_items: i64 = query_scalar!(
            "
            SELECT COUNT(*)
            FROM collage c
            WHERE (c.name ILIKE '%' || $1 || '%')
            ",
            form.name,
        )
        .fetch_one(self.borrow())
        .await
        .unwrap()
        .unwrap();

        let results = query_as_unchecked!(
            CollageSearchResult,
            r#"
            SELECT
                c.id,
                c.created_at,
                c.created_by_id,
                ROW(u.id, u.username, u.warned, u.banned) AS created_by,
                c.name,
                c.cover,
                c.description,
                c.tags,
                c.category,
                COUNT(ce.id) AS entries_amount,
                MAX(ce.created_at) AS last_entry_at
            FROM
                collage c
            JOIN
                users u ON c.created_by_id = u.id
            LEFT JOIN
                collage_entry ce ON c.id = ce.collage_id
            WHERE
                (c.name ILIKE '%' || $1 || '%')
            GROUP BY
                c.id, u.id
            ORDER BY
                c.created_at DESC
            OFFSET $2
            LIMIT $3
            "#,
            form.name,
            offset as i64,
            form.page_size as i64
        )
        .fetch_all(self.borrow())
        .await?;

        Ok(PaginatedResults {
            results,
            total_items,
            page: form.page,
            page_size: form.page_size,
        })
    }

    pub async fn search_collages_lite(
        &self,
        form: &SearchCollagesLiteQuery,
    ) -> Result<Vec<CollageLite>> {
        let results = sqlx::query_as!(
            CollageLite,
            r#"
                SELECT
                    c.id,
                    c.name,
                    c.cover
                FROM
                    collage c
                WHERE
                    (c.name ILIKE '%' || $1 || '%')
                ORDER BY
                    CASE
                        -- Exact Match: Highest priority
                        WHEN c.name = $1 THEN 1
                        -- Starts With Match (Prefix): Second highest priority
                        WHEN c.name ILIKE $1 || '%' THEN 2
                        -- Anywhere Match: Lowest priority (or all remaining)
                        ELSE 3
                    END
                LIMIT $2
                "#,
            form.name,
            form.results_amount as i16
        )
        .fetch_all(self.borrow())
        .await?;

        Ok(results)
    }
}
