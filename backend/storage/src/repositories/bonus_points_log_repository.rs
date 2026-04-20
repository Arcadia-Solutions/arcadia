use crate::{
    connection_pool::ConnectionPool,
    models::{
        bonus_points_log::{
            BonusPointsLog, BonusPointsLogAction, BonusPointsLogOrderByColumn,
            SearchBonusPointsLogsQuery,
        },
        common::{OrderByDirection, PaginatedResults},
    },
};
use arcadia_common::error::Result;
use sqlx::{PgPool, Postgres, Transaction};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn log_bonus_points_change_tx(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i32,
        action: BonusPointsLogAction,
        amount: i64,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                INSERT INTO bonus_points_logs (user_id, action, amount)
                VALUES ($1, $2, $3)
            "#,
            user_id,
            action as BonusPointsLogAction,
            amount
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    pub async fn award_bonus_points(
        &self,
        user_id: i32,
        action: BonusPointsLogAction,
        amount: i64,
    ) -> Result<()> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        sqlx::query!(
            r#"
                UPDATE users
                SET bonus_points = bonus_points + $2
                WHERE id = $1
            "#,
            user_id,
            amount
        )
        .execute(&mut *tx)
        .await?;

        Self::log_bonus_points_change_tx(&mut tx, user_id, action, amount).await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn search_bonus_points_logs(
        &self,
        user_id: i32,
        query: &SearchBonusPointsLogsQuery,
    ) -> Result<PaginatedResults<BonusPointsLog>> {
        let limit = query.page_size as i64;
        let offset = ((query.page - 1) * query.page_size) as i64;

        let order_by_column = match query.order_by_column {
            BonusPointsLogOrderByColumn::CreatedAt => "created_at",
            BonusPointsLogOrderByColumn::Amount => "amount",
            BonusPointsLogOrderByColumn::Action => "action",
        };
        let direction = match query.order_by_direction {
            OrderByDirection::Asc => "ASC",
            OrderByDirection::Desc => "DESC",
        };

        let actions: Vec<BonusPointsLogAction> = query
            .actions
            .split(',')
            .filter(|part| !part.is_empty())
            .filter_map(|part| {
                serde_json::from_value(serde_json::Value::String(part.to_owned())).ok()
            })
            .collect();

        let list_query = format!(
            r#"
                SELECT created_at, user_id, action, amount
                FROM bonus_points_logs
                WHERE user_id = $1
                  AND created_at >= $2
                  AND created_at <= $3
                  AND (cardinality($4::bonus_points_log_action_enum[]) = 0 OR action = ANY($4))
                ORDER BY {order_by_column} {direction}, created_at DESC
                LIMIT $5 OFFSET $6
            "#
        );

        let results: Vec<BonusPointsLog> = sqlx::query_as(&list_query)
            .bind(user_id)
            .bind(query.from_date)
            .bind(query.to_date)
            .bind(&actions)
            .bind(limit)
            .bind(offset)
            .fetch_all(<ConnectionPool as Borrow<PgPool>>::borrow(self))
            .await?;

        let total_items = sqlx::query_scalar!(
            r#"
                SELECT COUNT(*) AS "count!"
                FROM bonus_points_logs
                WHERE user_id = $1
                  AND created_at >= $2
                  AND created_at <= $3
                  AND (cardinality($4::bonus_points_log_action_enum[]) = 0 OR action = ANY($4))
            "#,
            user_id,
            query.from_date,
            query.to_date,
            actions.as_slice() as &[BonusPointsLogAction]
        )
        .fetch_one(<ConnectionPool as Borrow<PgPool>>::borrow(self))
        .await?;

        Ok(PaginatedResults {
            results,
            page: query.page,
            page_size: query.page_size,
            total_items,
        })
    }
}
