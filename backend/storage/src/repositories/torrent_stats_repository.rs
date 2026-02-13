use crate::{
    connection_pool::ConnectionPool,
    models::torrent_stats::{TorrentStatsDataPoint, TorrentStatsQuery, TorrentStatsResponse},
};
use arcadia_common::error::Result;
use std::borrow::Borrow;

impl ConnectionPool {
    /// returns results either from the beginning of the selected period or from the first uploaded torrent (whichever is the earliest)
    /// until the end of the selected period, filling with zero values if needed
    pub async fn get_torrent_stats(
        &self,
        query: &TorrentStatsQuery,
    ) -> Result<TorrentStatsResponse> {
        let data = sqlx::query_as!(
            TorrentStatsDataPoint,
            r#"
            WITH periods AS (
                SELECT generate_series(
                    date_trunc($3, $1::DATE::TIMESTAMP),
                    date_trunc($3, ($2::DATE - INTERVAL '1 day')::TIMESTAMP),
                    ('1 ' || $3)::INTERVAL
                ) AS period
            ),
            stats AS (
                SELECT
                    date_trunc($3, t.created_at)::TIMESTAMP AS period,
                    COUNT(*)::BIGINT AS count,
                    COALESCE(SUM(t.size), 0)::BIGINT AS total_size,
                    CASE $4
                        WHEN 'video_resolution' THEN t.video_resolution::TEXT
                        WHEN 'video_codec' THEN t.video_codec::TEXT
                        WHEN 'audio_codec' THEN t.audio_codec::TEXT
                        WHEN 'audio_channels' THEN t.audio_channels::TEXT
                        WHEN 'audio_bitrate_sampling' THEN t.audio_bitrate_sampling::TEXT
                        WHEN 'container' THEN t.container
                        WHEN 'source' THEN eg.source::TEXT
                        WHEN 'content_type' THEN tg.content_type::TEXT
                        WHEN 'category' THEN tg.category::TEXT
                        WHEN 'platform' THEN tg.platform::TEXT
                        WHEN 'original_language' THEN tg.original_language::TEXT
                        WHEN 'country_from' THEN tg.country_from
                        ELSE NULL
                    END AS attribute_value
                FROM torrents t
                JOIN edition_groups eg ON t.edition_group_id = eg.id
                JOIN title_groups tg ON eg.title_group_id = tg.id
                WHERE t.deleted_at IS NULL
                  AND t.created_at >= $1::DATE
                  AND t.created_at < $2::DATE
                  AND CASE $4
                        WHEN 'video_resolution' THEN t.video_resolution IS NOT NULL
                        WHEN 'video_codec' THEN t.video_codec IS NOT NULL
                        WHEN 'audio_codec' THEN t.audio_codec IS NOT NULL
                        WHEN 'audio_channels' THEN t.audio_channels IS NOT NULL
                        WHEN 'audio_bitrate_sampling' THEN t.audio_bitrate_sampling IS NOT NULL
                        WHEN 'container' THEN t.container IS NOT NULL
                        WHEN 'source' THEN eg.source IS NOT NULL
                        WHEN 'content_type' THEN tg.content_type IS NOT NULL
                        WHEN 'category' THEN tg.category IS NOT NULL
                        WHEN 'platform' THEN tg.platform IS NOT NULL
                        WHEN 'original_language' THEN tg.original_language IS NOT NULL
                        WHEN 'country_from' THEN tg.country_from IS NOT NULL
                        ELSE TRUE
                      END
                GROUP BY period, attribute_value
            ),
            attribute_values AS (
                SELECT DISTINCT attribute_value FROM stats WHERE attribute_value IS NOT NULL
            )
            SELECT
                p.period::TIMESTAMP AS "period!",
                COALESCE(s.count, 0)::BIGINT AS "count!",
                COALESCE(s.total_size, 0)::BIGINT AS "total_size!",
                av.attribute_value
            FROM periods p
            LEFT JOIN attribute_values av ON TRUE
            LEFT JOIN stats s ON p.period = s.period
                AND s.attribute_value IS NOT DISTINCT FROM av.attribute_value
            WHERE p.period >= COALESCE(
                date_trunc(
                    $3,
                    (SELECT MIN(created_at) FROM torrents WHERE deleted_at IS NULL)
                )::TIMESTAMP,
                p.period
            )
            ORDER BY p.period, av.attribute_value
            "#,
            query.from,
            query.to,
            &query.interval.to_string(),
            &query.group_by.to_string(),
        )
        .fetch_all(self.borrow())
        .await?;

        let unique_uploaders: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(DISTINCT t.created_by_id)::BIGINT AS "unique_uploaders!"
            FROM torrents t
            WHERE t.deleted_at IS NULL
              AND t.created_at >= $1::DATE
              AND t.created_at < $2::DATE
            "#,
            query.from,
            query.to,
        )
        .fetch_one(self.borrow())
        .await?;

        Ok(TorrentStatsResponse {
            unique_uploaders,
            data,
        })
    }
}
