use crate::{
    connection_pool::ConnectionPool,
    models::gift::{Gift, UserCreatedGift},
};
use arcadia_common::error::{Error, Result};
use sqlx::{PgPool, Postgres, Transaction};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_gift(&self, gift: &UserCreatedGift, current_user_id: i32) -> Result<Gift> {
        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        Self::decrement_bonus_points_and_freeleech_tokens(
            &mut tx,
            current_user_id,
            gift.bonus_points,
            gift.freeleech_tokens,
        )
        .await?;

        Self::increment_bonus_points_and_freeleech_tokens(
            &mut tx,
            gift.receiver_id,
            gift.bonus_points,
            gift.freeleech_tokens,
        )
        .await?;

        let gift = sqlx::query_as!(
            Gift,
            r#"
                INSERT INTO gifts (message, sender_id, receiver_id, bonus_points, freeleech_tokens)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, sent_at, message, sender_id, receiver_id, bonus_points, freeleech_tokens
            "#,
            gift.message,
            current_user_id,
            gift.receiver_id,
            gift.bonus_points,
            gift.freeleech_tokens
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::CouldNotCreateGift)?;

        tx.commit().await?;

        Ok(gift)
    }

    pub async fn decrement_bonus_points_and_freeleech_tokens(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i32,
        bonus_points: i64,
        freeleech_tokens: i32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
              UPDATE users SET bonus_points = bonus_points - $1,
              freeleech_tokens = freeleech_tokens - $2
              WHERE id = $3
            "#,
            bonus_points,
            freeleech_tokens,
            user_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    async fn increment_bonus_points_and_freeleech_tokens(
        tx: &mut Transaction<'_, Postgres>,
        user_id: i32,
        bonus_points: i64,
        freeleech_tokens: i32,
    ) -> Result<()> {
        sqlx::query!(
            r#"
              UPDATE users SET bonus_points = bonus_points + $1,
              freeleech_tokens = freeleech_tokens + $2
              WHERE id = $3
            "#,
            bonus_points,
            freeleech_tokens,
            user_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
