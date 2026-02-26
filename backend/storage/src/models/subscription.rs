use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::models::common::OrderByDirection;

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct SearchSubscriptionsQuery {
    pub page: u32,
    pub page_size: u32,
    pub order_by_direction: OrderByDirection,
}
