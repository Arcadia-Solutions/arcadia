use crate::{
    connection_pool::ConnectionPool,
    models::shop::{ShopItem, ShopPurchase},
};
use arcadia_common::error::{Error, Result};
use sqlx::{PgPool, Postgres, Transaction};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn purchase_upload(
        &self,
        user_id: i32,
        bytes: i64,
        bonus_points_cost: i64,
    ) -> Result<ShopPurchase> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        Self::decrement_bonus_points_and_freeleech_tokens(&mut tx, user_id, bonus_points_cost, 0)
            .await?;
        Self::add_upload(&mut tx, user_id, bytes).await?;

        let purchase = sqlx::query_as!(
            ShopPurchase,
            r#"
                INSERT INTO shop_purchases (user_id, item_type, bonus_points_spent, quantity, extra_info)
                VALUES ($1, $2, $3, $4, NULL)
                RETURNING id, user_id, purchased_at, item_type as "item_type: ShopItem", bonus_points_spent, quantity, extra_info
            "#,
            user_id,
            ShopItem::Upload as ShopItem,
            bonus_points_cost,
            bytes
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::CouldNotCreateShopPurchase)?;

        tx.commit().await?;

        Ok(purchase)
    }

    pub async fn purchase_freeleech_tokens(
        &self,
        user_id: i32,
        quantity: i32,
        bonus_points_cost: i64,
    ) -> Result<ShopPurchase> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        Self::decrement_bonus_points_and_freeleech_tokens(&mut tx, user_id, bonus_points_cost, 0)
            .await?;
        Self::add_freeleech_tokens(&mut tx, user_id, quantity).await?;

        let purchase = sqlx::query_as!(
            ShopPurchase,
            r#"
                INSERT INTO shop_purchases (user_id, item_type, bonus_points_spent, quantity, extra_info)
                VALUES ($1, $2, $3, $4, NULL)
                RETURNING id, user_id, purchased_at, item_type as "item_type: ShopItem", bonus_points_spent, quantity, extra_info
            "#,
            user_id,
            ShopItem::FreeleechTokens as ShopItem,
            bonus_points_cost,
            quantity as i64
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::CouldNotCreateShopPurchase)?;

        tx.commit().await?;

        Ok(purchase)
    }

    pub async fn record_promotion_purchase(
        &self,
        user_id: i32,
        new_class_name: &str,
        bonus_points_cost: i64,
    ) -> Result<ShopPurchase> {
        let purchase = sqlx::query_as!(
            ShopPurchase,
            r#"
                INSERT INTO shop_purchases (user_id, item_type, bonus_points_spent, quantity, extra_info)
                VALUES ($1, $2, $3, 1, $4)
                RETURNING id, user_id, purchased_at, item_type as "item_type: ShopItem", bonus_points_spent, quantity, extra_info
            "#,
            user_id,
            ShopItem::Promotion as ShopItem,
            bonus_points_cost,
            new_class_name
        )
        .fetch_one(self.borrow())
        .await
        .map_err(Error::CouldNotCreateShopPurchase)?;

        Ok(purchase)
    }

    pub async fn user_has_enough_bonus_points(&self, user_id: i32, required: i64) -> Result<bool> {
        let result = sqlx::query_scalar!(
            r#"
                SELECT bonus_points >= $1 as "has_enough!"
                FROM users
                WHERE id = $2
            "#,
            required,
            user_id
        )
        .fetch_one(self.borrow())
        .await?;

        Ok(result)
    }

    pub async fn get_shop_purchase_history(&self, user_id: i32) -> Result<Vec<ShopPurchase>> {
        let purchases = sqlx::query_as!(
            ShopPurchase,
            r#"
                SELECT id, user_id, purchased_at, item_type as "item_type: ShopItem", bonus_points_spent, quantity, extra_info
                FROM shop_purchases
                WHERE user_id = $1
                ORDER BY purchased_at DESC
            "#,
            user_id
        )
        .fetch_all(self.borrow())
        .await
        .map_err(Error::CouldNotGetShopPurchaseHistory)?;

        Ok(purchases)
    }

    async fn add_upload(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i32,
        bytes: i64,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE users
                SET uploaded = uploaded + $1
                WHERE id = $2
            "#,
            bytes,
            user_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    async fn add_freeleech_tokens(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i32,
        quantity: i32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
                UPDATE users
                SET freeleech_tokens = freeleech_tokens + $1
                WHERE id = $2
            "#,
            quantity,
            user_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
