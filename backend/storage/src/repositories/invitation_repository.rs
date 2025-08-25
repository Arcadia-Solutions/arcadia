use crate::{
    connection_pool::ConnectionPool,
    models::invitation::{Invitation, SentInvitation},
};
use arcadia_common::error::{Error, Result};
use rand::{
    distr::{Alphanumeric, SampleString},
    rng,
};
use sqlx::{PgPool, Postgres, Transaction};
use std::borrow::Borrow;

impl ConnectionPool {
    pub async fn create_invitation(
        &self,
        invitation: &SentInvitation,
        current_user_id: i64,
    ) -> Result<Invitation> {
        // TODO: retry if invitation_key already exists
        let invitation_key: String = Alphanumeric.sample_string(&mut rng(), 50);

        let mut tx = <ConnectionPool as Borrow<PgPool>>::borrow(self)
            .begin()
            .await?;

        let _ = Self::decrement_invitations_available(&mut tx, current_user_id).await;

        // TODO: make invitation expiration configurable
        // TODO: make sure no invitation/user exists for this email address
        let created_invitation = sqlx::query_as!(
            Invitation,
            r#"
                INSERT INTO invitations (message, invitation_key, sender_id, receiver_email, expires_at, user_application_id)
                VALUES ($1, $2, $3, $4, NOW() + INTERVAL '3 days', $5)
                RETURNING *
            "#,
            invitation.message,
            invitation_key,
            current_user_id,
            invitation.receiver_email,
            invitation.user_application_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(Error::CouldNotCreateInvitation)?;

        if invitation.user_application_id.is_some() {
            sqlx::query!(
                r#"
                UPDATE user_applications
                SET status = 'accepted'
                WHERE id = $1;
            "#,
                invitation.user_application_id
            )
            .execute(&mut *tx)
            .await
            .map_err(Error::CouldNotCreateInvitation)?;
        }

        tx.commit().await?;

        Ok(created_invitation)
    }

    pub async fn does_unexpired_invitation_exist(
        &self,
        invitation_key: &str,
    ) -> Result<Invitation> {
        let invitation = sqlx::query_as!(
            Invitation,
            r#"
              SELECT * FROM invitations
              WHERE invitation_key = $1
              AND expires_at > NOW()
            "#,
            invitation_key
        )
        .fetch_one(self.borrow())
        .await
        .map_err(|_| Error::InvitationKeyInvalid)?;

        Ok(invitation)
    }

    pub async fn decrement_invitations_available(
        tx: &mut Transaction<'_, Postgres>,
        current_user_id: i64,
    ) -> Result<()> {
        sqlx::query!(
            r#"
              UPDATE users SET invitations = invitations - 1
              WHERE id = $1
            "#,
            current_user_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}
