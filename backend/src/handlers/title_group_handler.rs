use actix_web::{HttpResponse, web};
use futures::future::join_all;
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{
    Arcadia, Result,
    handlers::scrapers::tmdb::get_tmdb_rating,
    models::{
        title_group::{
            ContentType, PublicRating, TitleGroup, TitleGroupAndAssociatedData, TitleGroupLite,
            UserCreatedTitleGroup,
        },
        user::User,
    },
    repositories::{
        artist_repository::create_artists_affiliation,
        title_group_repository::{
            create_title_group, find_title_group, find_title_group_info_lite,
        },
    },
};

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

    let created_title_group = create_title_group(&arc.pool, &form, &ratings, &current_user).await?;

    if !form.affiliated_artists.is_empty() {
        for artist in &mut form.affiliated_artists {
            artist.title_group_id = created_title_group.id
        }

        let _ = create_artists_affiliation(&arc.pool, &form.affiliated_artists, current_user.id)
            .await?;
    }

    Ok(HttpResponse::Created().json(created_title_group))
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
    let title_group = find_title_group(&arc.pool, query.id, &current_user).await?;

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
    let title_group = find_title_group_info_lite(&arc.pool, Some(query.id), None, &None, 1).await?;

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
    let title_groups =
        find_title_group_info_lite(&arc.pool, None, Some(&query.name), &query.content_type, 5)
            .await?;

    Ok(HttpResponse::Ok().json(title_groups))
}
