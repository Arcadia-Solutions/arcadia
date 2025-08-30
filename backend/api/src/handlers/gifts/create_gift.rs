use crate::{middlewares::jwt_middleware::Authdata, Arcadia};
use actix_web::{web, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::gift::{Gift, UserCreatedGift};

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
pub async fn exec(
    gift: web::Json<UserCreatedGift>,
    arc: web::Data<Arcadia>,
    user: Authdata,
) -> Result<HttpResponse> {
    let current_user = arc.pool.find_user_with_id(user.sub).await?;
    if current_user.bonus_points < gift.bonus_points {
        return Err(Error::NotEnoughBonusPointsAvailable);
    }
    if current_user.freeleech_tokens < gift.freeleech_tokens {
        return Err(Error::NotEnoughFreeleechTokensAvailable);
    }

    let gift = arc.pool.create_gift(&gift, user.sub).await?;

    Ok(HttpResponse::Created().json(gift))
}
