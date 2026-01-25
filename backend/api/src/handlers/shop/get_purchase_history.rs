use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::{models::shop::ShopPurchase, redis::RedisPoolInterface};

#[utoipa::path(
    get,
    operation_id = "Get shop purchase history",
    tag = "Shop",
    path = "/api/shop/history",
    security(("http" = ["Bearer"])),
    responses(
        (status = 200, description = "User's shop purchase history", body = Vec<ShopPurchase>),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    current_user: Authdata,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    let purchases = arc.pool.get_shop_purchase_history(current_user.sub).await?;

    Ok(HttpResponse::Ok().json(purchases))
}
