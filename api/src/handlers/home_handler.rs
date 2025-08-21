use crate::Arcadia;
use actix_web::{HttpResponse, web};
use arcadia_storage::{
    models::{
        forum::ForumPostAndThreadName, home_stats::HomeStats, title_group::TitleGroupLite
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use arcadia_common::error::Result;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HomePage {
    recent_announcements: Vec<ForumPostAndThreadName>,
    stats: HomeStats,
    latest_uploads: Vec<TitleGroupLite>,
}

#[utoipa::path(
    get,
    path = "/api/home",
    responses(
        (status = 200, description = "", body=HomePage),
    )
)]
pub async fn get_home(arc: web::Data<Arcadia>) -> Result<HttpResponse> {
    let recent_announcements = arc.pool.find_first_thread_posts_in_sub_category(1, 5).await?;
    let stats = arc.pool.find_home_stats().await?;
    let latest_uploads_in_title_groups =
        arc.pool.find_title_group_info_lite(None, Some(""), &None, 5).await?;

    Ok(HttpResponse::Created().json(json!({
        "recent_announcements":recent_announcements,
        "stats": stats,
        "latest_uploads": latest_uploads_in_title_groups,
    })))
}
