use crate::{
    connection_pool::ConnectionPool,
    models::{
        collage::{
            Collage, CollageCategory, CollageEntry, CollageSearchResult, CollageType,
            SearchCollagesQuery, UserCreatedCollage, UserCreatedCollageEntry,
        },
        common::PaginatedResults,
    },
};
use arcadia_common::error::{Error, Result};
use serde_json::Value;
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
                INSERT INTO collage (created_by_id, name, cover, description, tags, category, collage_type)
                VALUES ($1, $2, $3, $4, $5, $6::collage_category_enum, $7::collage_type_enum)
                RETURNING id, created_at, created_by_id, name, cover, description, tags, collage_type as "collage_type: CollageType",
                                          category as "category: CollageCategory"
            "#,
            user_id,
            collage.name,
            collage.cover,
            collage.description,
            &collage.tags,
            collage.category as _,
            collage.collage_type as _
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
                        artist_id,
                        entity_id,
                        title_group_id,
                        master_group_id,
                        collage_id,
                        note
                    )
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    RETURNING *
                "#,
                user_id,
                entry.artist_id,
                entry.entity_id,
                entry.title_group_id,
                entry.master_group_id,
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

    pub async fn find_collage_and_associated_data(&self, collage_id: &i64) -> Result<Value> {
        let collage_data = sqlx::query!(
            r#"
            WITH collage_entries_data AS (
                SELECT
                    ce.collage_id,
                    COALESCE(
                        jsonb_agg(
                            jsonb_build_object(
                                'id', ce.id,
                                'created_at', ce.created_at,
                                'created_by_id', ce.created_by_id,
                                'artist_id', ce.artist_id,
                                'artist', CASE
                                            WHEN a.id IS NOT NULL THEN
                                                jsonb_build_object(
                                                    'id', a.id,
                                                    'name', a.name,
                                                    'pictures', a.pictures
                                                )
                                            ELSE NULL
                                        END,
                                'entity_id', ce.entity_id,
                                'entity', CASE
                                            WHEN e.id IS NOT NULL THEN
                                                jsonb_build_object(
                                                    'id', e.id,
                                                    'name', e.name,
                                                    'pictures', e.pictures
                                                )
                                            ELSE NULL
                                        END,
                                'title_group_id', ce.title_group_id,
                                'title_group', tgd.title_group_data,
                                'master_group_id', ce.master_group_id,
                                'master_group', CASE
                                                    WHEN mg.id IS NOT NULL THEN
                                                        jsonb_build_object(
                                                            'id', mg.id,
                                                            'name', mg.name
                                                        )
                                                    ELSE NULL
                                                END,
                                'collage_id', ce.collage_id,
                                'note', ce.note
                            )
                        ), '[]'::jsonb
                    ) AS entries
                FROM collage_entry ce
                LEFT JOIN artists a ON ce.artist_id = a.id
                LEFT JOIN entities e ON ce.entity_id = e.id
                LEFT JOIN master_groups mg ON ce.master_group_id = mg.id
                LEFT JOIN get_title_groups_and_edition_group_and_torrents_lite AS tgd ON tgd.title_group_id = ce.title_group_id
                WHERE ce.collage_id = $1
                GROUP BY ce.collage_id
            )
            SELECT
                jsonb_build_object(
                    'collage', to_jsonb(c),
                    'entries', COALESCE(ced.entries, '[]'::jsonb)
                ) AS collage_data
            FROM collage c
            LEFT JOIN collage_entries_data ced ON ced.collage_id = c.id
            WHERE c.id = $1;
            "#,
            collage_id,
        )
        .fetch_one(self.borrow())
        .await?;

        Ok(collage_data.collage_data.unwrap())
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
                c.collage_type,
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
}
