use actix_web::{web, HttpResponse};
use arcadia_storage::models::title_group::{
    ContentType, EditedTitleGroup, PublicRating, TitleGroup, TitleGroupAndAssociatedData,
    TitleGroupLite, UserCreatedTitleGroup,
};
use futures::future::join_all;
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{
    handlers::{scrapers::tmdb::get_tmdb_rating, User},
    Arcadia,
};
use arcadia_common::error::{Error, Result};

#[utoipa::path(
    post,
    path = "/api/title-group",
    responses(
        (status = 200, description = "Successfully created the title_group", body=TitleGroup),
    )
)]
pub async fn add_title_group(
    mut form: web::Json<UserCreatedTitleGroup>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let rating_futures: Vec<_> = form
        .external_links
        .iter()
        .filter(|link| link.contains("https://www.themoviedb.org/"))
        .map(|link| get_tmdb_rating(link, arc.tmdb_api_key.clone().unwrap()))
        .collect();
    let ratings: Vec<PublicRating> = join_all(rating_futures)
        .await
        .into_iter()
        .filter_map(Result::ok)
        .collect();

    let created_title_group = arc
        .pool
        .create_title_group(&form, &ratings, &current_user)
        .await?;

    if !form.affiliated_artists.is_empty() {
        for artist in &mut form.affiliated_artists {
            artist.title_group_id = created_title_group.id
        }

        let _ = arc
            .pool
            .create_artists_affiliation(&form.affiliated_artists, current_user.id)
            .await?;
    }

    Ok(HttpResponse::Created().json(created_title_group))
}

#[utoipa::path(
    put,
    path = "/api/title-group",
    responses(
        (status = 200, description = "Successfully edited the title group", body=TitleGroup),
    )
)]
pub async fn edit_title_group(
    form: web::Json<EditedTitleGroup>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let title_group = arc.pool.find_title_group(form.id).await?;

    if title_group.created_by_id == current_user.id || current_user.class == "staff" {
        let updated_title_group = arc.pool.update_title_group(&form, title_group.id).await?;
        Ok(HttpResponse::Ok().json(updated_title_group))
    } else {
        Err(Error::InsufficientPrivileges)
    }
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetTitleGroupQuery {
    id: i64,
}

#[utoipa::path(
    get,
    path = "/api/title-group",
    params(GetTitleGroupQuery),
    responses(
        (status = 200, description = "Successfully got the title_group", body=TitleGroupAndAssociatedData),
    )
)]
pub async fn get_title_group(
    arc: web::Data<Arcadia>,
    query: web::Query<GetTitleGroupQuery>,
    current_user: User,
) -> Result<HttpResponse> {
    let title_group = arc
        .pool
        .find_title_group_hierarchy(query.id, &current_user)
        .await?;

    Ok(HttpResponse::Ok().json(title_group))
}

pub type GetTitleGroupLiteQuery = GetTitleGroupQuery;

#[utoipa::path(
    get,
    path = "/api/title-group/lite",
    params(GetTitleGroupLiteQuery),
    responses(
        (status = 200, description = "Successfully got the title_group (lite info)", body=TitleGroupLite),
    )
)]
pub async fn get_title_group_info_lite(
    arc: web::Data<Arcadia>,
    query: web::Query<GetTitleGroupLiteQuery>,
) -> Result<HttpResponse> {
    let title_group = arc
        .pool
        .find_title_group_info_lite(Some(query.id), None, &None, 1)
        .await?;

    Ok(HttpResponse::Ok().json(title_group))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchTitleGroupLiteQuery {
    name: String,
    content_type: Option<ContentType>,
}

#[utoipa::path(
    get,
    path = "/api/search/title-group/lite",
    params(SearchTitleGroupLiteQuery),
    responses(
        (status = 200, description = "Returns title groups with their name containing the provided string, only the 5 first matches", body=Vec<TitleGroupLite>),
    )
)]
pub async fn search_title_group_info_lite(
    arc: web::Data<Arcadia>,
    query: web::Query<SearchTitleGroupLiteQuery>,
) -> Result<HttpResponse> {
    let title_groups = arc
        .pool
        .find_title_group_info_lite(None, Some(&query.name), &query.content_type, 5)
        .await?;

    Ok(HttpResponse::Ok().json(title_groups))
}
