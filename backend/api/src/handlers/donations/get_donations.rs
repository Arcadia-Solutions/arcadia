use crate::{middlewares::auth_middleware::Authdata, Arcadia};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{common::PaginatedResults, donation::Donation, user::UserClass},
    redis::RedisPoolInterface,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct GetDonationsQuery {
    pub page_size: Option<i64>,
    pub page: Option<i64>,
}

#[utoipa::path(
    get,
    operation_id = "Get donations",
    tag = "Donation",
    path = "/api/donations",
    params(
        ("page_size" = Option<i64>, Query, description = "Maximum number of donations to return (default: 50)"),
        ("page" = Option<i64>, Query, description = "Page (default: 1)")
    ),
    responses(
        (status = 200, description = "Successfully retrieved donations", body = PaginatedResults<Donation>),
        (status = 403, description = "Forbidden - Only staff members can view donations")
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    arc: Data<Arcadia<R>>,
    user: Authdata,
    query: Query<GetDonationsQuery>,
) -> Result<HttpResponse> {
    if user.class != UserClass::Staff {
        return Err(Error::InsufficientPrivileges);
    }

    let donations = arc
        .pool
        .find_donations(query.page_size.unwrap_or(50), query.page.unwrap_or(1))
        .await?;

    Ok(HttpResponse::Ok().json(donations))
}
