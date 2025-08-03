use rand::{
    distr::{Alphanumeric, SampleString},
    rng,
};
use sqlx::PgPool;

use crate::{
    Error, Result,
    models::api_token::{APIToken, UserCreatedAPIToken},
};

pub async fn create_api_token(
    pool: &PgPool,
    send_token: &UserCreatedAPIToken,
    current_user_id: i64,
) -> Result<APIToken> {
    let api_key: String = Alphanumeric.sample_string(&mut rng(), 40);

    let mut tx = pool.begin().await?;

    let api_token = sqlx::query_as!(
        APIToken,
        r#"
            INSERT INTO api_keys (name, value, user_id)
            VALUES ($1, $2, $3)
            RETURNING *
        "#,
        send_token.name,
        api_key,
        current_user_id
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(Error::CouldNotCreateInvitation)?;

    tx.commit().await?;

    Ok(api_token)
}
