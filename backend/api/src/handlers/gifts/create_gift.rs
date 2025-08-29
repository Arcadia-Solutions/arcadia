use crate::{handlers::User, Arcadia};
use actix_web::{
    web::{self, Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::gift::{Gift, UserCreatedGift},
    redis::RedisPoolInterface,
};

#[utoipa::path(
    post,
    operation_id = "Create gift",
    tag = "Gift",
    path = "/api/gifts",
    security(
      ("http" = ["Bearer"])
    ),
    responses(
        (status = 200, description = "Successfully sent the gift", body=Gift),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    gift: Json<UserCreatedGift>,
    arc: Data<Arcadia<R>>,
    current_user: User,
) -> Result<HttpResponse> {
    if current_user.bonus_points < gift.bonus_points {
        return Err(Error::NotEnoughBonusPointsAvailable);
    }
    if current_user.freeleech_tokens < gift.freeleech_tokens {
        return Err(Error::NotEnoughFreeleechTokensAvailable);
    }

    let gift = arc.pool.create_gift(&gift, current_user.id).await?;

    Ok(HttpResponse::Created().json(gift))
}
