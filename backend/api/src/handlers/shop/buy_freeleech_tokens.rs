use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Json},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::shop::{BuyFreeleechTokensRequest, FreeleechTokenDiscountTier, ShopPurchase},
    redis::RedisPoolInterface,
    services::shop_service::calculate_freeleech_tokens_price,
};

#[utoipa::path(
    post,
    operation_id = "Buy freeleech tokens",
    tag = "Shop",
    path = "/api/shop/buy-freeleech-tokens",
    security(("http" = ["Bearer"])),
    request_body = BuyFreeleechTokensRequest,
    responses(
        (status = 201, description = "Successfully bought freeleech tokens", body = ShopPurchase),
        (status = 400, description = "Invalid quantity"),
        (status = 409, description = "Not enough bonus points"),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    request: Json<BuyFreeleechTokensRequest>,
    current_user: Authdata,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    if request.quantity < 1 {
        return Err(Error::InvalidShopPurchaseAmount);
    }

    let price_calculation = {
        let settings = arc.settings.lock().unwrap();
        let discount_tiers: Vec<FreeleechTokenDiscountTier> =
            serde_json::from_value(settings.shop_freeleech_token_discount_tiers.clone())?;
        calculate_freeleech_tokens_price(
            request.quantity,
            settings.shop_freeleech_token_base_price,
            &discount_tiers,
        )
    };

    if !arc
        .pool
        .user_has_enough_bonus_points(current_user.sub, price_calculation.final_price)
        .await?
    {
        return Err(Error::NotEnoughBonusPointsAvailable);
    }

    let purchase = arc
        .pool
        .purchase_freeleech_tokens(
            current_user.sub,
            request.quantity,
            price_calculation.final_price,
        )
        .await?;

    Ok(HttpResponse::Created().json(purchase))
}
