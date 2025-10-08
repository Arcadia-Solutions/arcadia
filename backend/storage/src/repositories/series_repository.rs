use crate::{
    connection_pool::ConnectionPool,
    models::series::{
        SearchSeriesQuery, Series, SeriesSearchResponse, SeriesSearchResult, UserCreatedSeries,
    },
};
use arcadia_common::error::{Error, Result};
use serde_json::Value;
use sqlx::{query_as_unchecked, query_scalar};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_series(&self, series: &UserCreatedSeries, user_id: i32) -> Result<Series> {
        let created_series = sqlx::query_as!(
            Series,
            r#"
                INSERT INTO series (name,description,created_by_id,covers,banners,tags)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING *
            "#,
            series.name,
            series.description,
            user_id,
            &series.covers,
            &series.banners,
            &series.tags
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateSeries)?;

        Ok(created_series)
    }

    pub async fn find_series(&self, series_id: &i64) -> Result<Value> {
        let found_series = sqlx::query!(
            r#"
            SELECT
                jsonb_build_object(
                    'series', to_jsonb(s),
                    'title_groups', COALESCE(
                        jsonb_agg(tgd.title_group_data),
                        '[]'::jsonb
                    )
                ) AS series_and_title_groups
            FROM
                series s
            LEFT JOIN
                title_groups tg ON s.id = tg.series_id
            LEFT JOIN
                get_title_groups_and_edition_group_and_torrents_lite() AS tgd ON tg.id = tgd.title_group_id
            WHERE
                s.id = $1
            GROUP BY
                s.id, s.*;
            "#,
            series_id
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::SeriesWithIdNotFound(*series_id))?;

        // Ok(serde_json::from_value(found_series.series_and_groups.unwrap()).unwrap())
        Ok(found_series.series_and_title_groups.unwrap())
    }

    pub async fn search_series(&self, form: &SearchSeriesQuery) -> Result<SeriesSearchResponse> {
        let offset = (form.page - 1) * form.page_size;

        let total_items: i64 = query_scalar!(
            r#"
            SELECT COUNT(*)
            FROM series s
            WHERE (s.name ILIKE '%' || $1 || '%')
            "#,
            form.name,
        )
        .fetch_one(self.borrow())
        .await
        .unwrap()
        .unwrap();

        let results = query_as_unchecked!(
            SeriesSearchResult,
            r#"
            SELECT
                s.id,
                s.name,
                s.created_at,
                s.created_by_id,
                s.covers,
                s.banners,
                s.tags,
                COALESCE(tg_count.cnt, 0) AS title_groups_amount
            FROM series s
            LEFT JOIN (
                SELECT series_id, COUNT(*) AS cnt
                FROM title_groups
                WHERE series_id IS NOT NULL
                GROUP BY series_id
            ) tg_count ON tg_count.series_id = s.id
            WHERE (s.name ILIKE '%' || $1 || '%')
            ORDER BY s.created_at DESC
            OFFSET $2
            LIMIT $3
            "#,
            form.name,
            offset as i64,
            form.page_size as i64
        )
        .fetch_all(self.borrow())
        .await?;

        Ok(SeriesSearchResponse {
            results,
            total_items,
        })
    }
}
