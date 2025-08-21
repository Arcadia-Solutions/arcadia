use crate::{
    Arcadia,
    handlers::UserId,
};
use actix_web::{HttpResponse, web};
use arcadia_storage::{
    models::artist::{
        AffiliatedArtistHierarchy, Artist, ArtistAndTitleGroupsLite, ArtistLite,
        UserCreatedAffiliatedArtist, UserCreatedArtist,
    },
    repositories::artist_repository::{
        create_artists, create_artists_affiliation, delete_artists_affiliation,
        find_artist_publications,find_artists_lite,
    },
};
use serde::Deserialize;
use serde_json::json;
use utoipa::{IntoParams, ToSchema};
use arcadia_common::error::Result;

#[utoipa::path(
    post,
    path = "/api/artists",
    responses(
        (status = 201, description = "Successfully created the artists, returned in the same order as the one sent.
            In the case of a db conflict (duplicate), the existing entry is returned (can be seen with the created_at attribute).", body=Vec<Artist>),
    )
)]
pub async fn add_artists(
    artists: web::Json<Vec<UserCreatedArtist>>,
    arc: web::Data<Arcadia>,
    current_user_id: UserId,
) -> Result<HttpResponse> {
    let artists = create_artists(arc.pool.borrow(), &artists, current_user_id.0).await?;

    Ok(HttpResponse::Created().json(artists))
}

#[utoipa::path(
    post,
    path = "/api/affiliated-artists",
    responses(
        (status = 200, description = "Successfully created the artist affiliations", body=Vec<AffiliatedArtistHierarchy>),
    )
)]
pub async fn add_affiliated_artists(
    artists: web::Json<Vec<UserCreatedAffiliatedArtist>>,
    arc: web::Data<Arcadia>,
    current_user_id: UserId,
) -> Result<HttpResponse> {
    let affiliations = create_artists_affiliation(arc.pool.borrow(), &artists, current_user_id.0).await?;

    Ok(HttpResponse::Created().json(affiliations))
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct RemoveAffiliatedArtistsQuery {
    affiliation_ids: Vec<i64>,
}

#[utoipa::path(
    delete,
    path = "/api/affiliated-artists",
    responses(
        (status = 200, description = "Successfully removed the artist affiliations"),
    )
)]
pub async fn remove_affiliated_artists(
    query: actix_web_lab::extract::Query<RemoveAffiliatedArtistsQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    // TODO: add protection based on user class
    delete_artists_affiliation(arc.pool.borrow(), &query.affiliation_ids).await?;

    Ok(HttpResponse::Ok().json(json!({"result": "success"})))
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetArtistPublicationsQuery {
    id: i64,
}

#[utoipa::path(
    get,
    path = "/api/artist",
    params (GetArtistPublicationsQuery),
    responses(
        (status = 200, description = "Successfully got the artist's pulications", body=ArtistAndTitleGroupsLite),
    )
)]
pub async fn get_artist_publications(
    query: web::Query<GetArtistPublicationsQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let artist_publication = find_artist_publications(arc.pool.borrow(), &query.id).await?;

    Ok(HttpResponse::Ok().json(artist_publication))
}

#[derive(Debug, Deserialize, ToSchema, IntoParams)]
pub struct GetArtistLiteQuery {
    name: String,
}

#[utoipa::path(
    get,
    path = "/api/search/artist/lite",
    params (GetArtistLiteQuery),
    description = "Case insensitive",
    responses(
        (status = 200, description = "Successfully got the artists and some data about them", body=Vec<ArtistLite>),
    )
)]
pub async fn get_artists_lite(
    query: web::Query<GetArtistLiteQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let artists = find_artists_lite(arc.pool.borrow(), &query.name).await?;

    Ok(HttpResponse::Ok().json(artists))
}
