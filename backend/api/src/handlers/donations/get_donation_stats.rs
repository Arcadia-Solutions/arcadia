use crate::Arcadia;
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::{models::donation::DonationStats, redis::RedisPoolInterface};

#[utoipa::path(
    get,
    operation_id = "Get donation stats",
    tag = "Donation",
    path = "/api/donations/stats",
    responses(
        (status = 200, description = "Successfully retrieved donation stats", body = DonationStats)
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(arc: Data<Arcadia<R>>) -> Result<HttpResponse> {
    let stats = arc.pool.get_donation_stats().await?;

    Ok(HttpResponse::Ok().json(stats))
}
