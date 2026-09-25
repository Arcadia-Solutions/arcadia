use crate::{
    connection_pool::ConnectionPool,
    models::user_stats::{
        TorrentClientStatsDataPoint, UserStatsDataPoint, UserStatsQuery, UserStatsResponse,
    },
};
use arcadia_common::error::Result;
use std::borrow::Borrow;

impl ConnectionPool {
    /// Returns the amount of registrations per period, alongside the total for that period.
    /// Periods with no registration are filled with zero values, starting from the first
    /// registered user.
    pub async fn get_users_stats(&self, query: &UserStatsQuery) -> Result<UserStatsResponse> {
        let data = sqlx::query_as!(
            UserStatsDataPoint,
            r#"
            WITH periods AS (
                SELECT generate_series(
                    date_trunc($3, $1::DATE::TIMESTAMP),
                    date_trunc($3, $2::DATE::TIMESTAMP),
                    ('1 ' || $3)::INTERVAL
                ) AS period
            ),
            stats AS (
                SELECT
                    date_trunc($3, u.created_at)::TIMESTAMP AS period,
                    COUNT(*)::BIGINT AS count
                FROM users u
                WHERE u.created_at >= $1::DATE
                  AND u.created_at < ($2::DATE + INTERVAL '1 day')
                GROUP BY period
            )
            SELECT
                p.period::TIMESTAMP AS "period!",
                COALESCE(s.count, 0)::BIGINT AS "count!"
            FROM periods p
            LEFT JOIN stats s ON p.period = s.period
            WHERE p.period >= COALESCE(
                date_trunc($3, (SELECT MIN(created_at) FROM users))::TIMESTAMP,
                p.period
            )
            ORDER BY p.period
            "#,
            query.from,
            query.to,
            &query.interval.to_string(),
        )
        .fetch_all(self.borrow())
        .await?;

        let new_users = data.iter().map(|data_point| data_point.count).sum();

        let torrent_clients = sqlx::query!(
            r#"
            SELECT
                agent,
                COUNT(*)::BIGINT AS count
            FROM peers
            WHERE agent IS NOT NULL
            GROUP BY agent
            ORDER BY count DESC
            "#,
        )
        .fetch_all(self.borrow())
        .await?
        .into_iter()
        .map(|row| TorrentClientStatsDataPoint {
            client: row.agent,
            count: row.count.unwrap_or(0),
        })
        .collect();

        Ok(UserStatsResponse {
            new_users,
            data,
            torrent_clients,
        })
    }
}
