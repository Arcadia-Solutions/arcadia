use crate::{
    handlers::scrapers::ExternalDBData, services::common_service::naive_date_to_utc_midnight,
    Arcadia,
};
use actix_web::{web, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::models::{
    edition_group::{create_default_edition_group, UserCreatedEditionGroup},
    title_group::{
        create_default_title_group, ContentType, ExternalDB, PublicRating, UserCreatedTitleGroup,
    },
};
use regex::Regex;
use serde::Deserialize;
use tmdb_api::client::reqwest::Client as ReqwestClient;
use tmdb_api::client::Client;
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetTMDBQuery {
    url: String,
}

async fn get_tmdb_movie_data(client: &Client<ReqwestClient>, id: u64) -> Result<ExternalDBData> {
    let tmdb_movie = client
        .get_movie_details(id, &Default::default())
        .await
        .unwrap();
    let mut title_group = UserCreatedTitleGroup {
        name: tmdb_movie.inner.original_title.clone(),
        name_aliases: (tmdb_movie.inner.title != tmdb_movie.inner.original_title)
            .then_some(vec![tmdb_movie.inner.original_title])
            .unwrap_or_default(),
        tags: tmdb_movie
            .genres
            .iter()
            .map(|g| g.name.clone().to_lowercase())
            .collect(),
        description: tmdb_movie.inner.overview,
        original_language: Some(tmdb_movie.inner.original_language),
        original_release_date: tmdb_movie
            .inner
            .release_date
            .map(naive_date_to_utc_midnight),
        covers: vec![tmdb_movie
            .inner
            .poster_path
            .map(|path| format!("https://image.tmdb.org/t/p/w1280{path}"))
            .unwrap_or("".to_string())],
        content_type: ContentType::Movie,
        ..create_default_title_group()
    };

    if let Some(link) = tmdb_movie
        .imdb_id
        .map(|id| format!("https://www.imdb.com/title/{id}"))
    {
        title_group.external_links = vec![link];
    }

    let edition_group = UserCreatedEditionGroup {
        release_date: title_group.original_release_date.unwrap_or_default(),
        ..create_default_edition_group()
    };
    Ok(ExternalDBData {
        title_group: Some(title_group),
        edition_group: Some(edition_group),
    })
}

#[utoipa::path(
    post,
    params(GetTMDBQuery),
    path = "/api/external_db/tmdb",
    responses(
        (status = 200, description = "", body=ExternalDBData),
    )
)]
pub async fn get_tmdb_data(
    query: web::Query<GetTMDBQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    if arc.tmdb_api_key.is_none() {
        return Err(Error::TMDBDataFetchingNotAvailable);
    }
    let (media_type, id) = extract_media_type_and_id(&query.url).unwrap();

    let client = Client::<ReqwestClient>::new(arc.tmdb_api_key.clone().unwrap());

    let mut external_db_data = match media_type {
        ContentType::Movie => get_tmdb_movie_data(&client, id).await?,
        ContentType::TVShow => todo!(),
        // should never happen
        _ => return Err(Error::InvalidTMDBUrl),
    };

    if let Some(title_group) = &mut external_db_data.title_group {
        title_group.external_links.push(query.url.clone());
    }

    Ok(HttpResponse::Ok().json(external_db_data))
}

pub async fn get_tmdb_rating(tmdb_url: &str, tmdb_api_key: String) -> Result<PublicRating> {
    let (media_type, id) = extract_media_type_and_id(tmdb_url).unwrap();

    let client = Client::<ReqwestClient>::new(tmdb_api_key);

    let rating = match media_type {
        ContentType::Movie => {
            let tmdb_movie = client
                .get_movie_details(id, &Default::default())
                .await
                .unwrap();
            PublicRating {
                service: ExternalDB::Tmdb,
                rating: tmdb_movie.inner.vote_average,
                votes: tmdb_movie.inner.vote_count as i64,
            }
        }
        ContentType::TVShow => {
            let tmdb_tv_show = client
                .get_tvshow_details(id, &Default::default())
                .await
                .unwrap();
            PublicRating {
                service: ExternalDB::Tmdb,
                rating: tmdb_tv_show.inner.vote_average,
                votes: tmdb_tv_show.inner.vote_count as i64,
            }
        }
        _ => return Err(Error::InvalidTMDBUrl),
    };

    Ok(rating)
}

fn extract_media_type_and_id(tmdb_url: &str) -> Result<(ContentType, u64)> {
    let re = Regex::new(r"themoviedb\.org/(movie|tv)/(\d+)(?:-|$)").unwrap();
    let captures = re.captures(tmdb_url).unwrap();

    let media_type_str = captures.get(1).unwrap().as_str();
    let media_type = match media_type_str {
        "movie" => ContentType::Movie,
        "tv" => ContentType::TVShow,
        _ => return Err(Error::InvalidTMDBUrl),
    };
    let id_str = captures.get(2).unwrap().as_str();
    let id = id_str.parse::<u64>().ok().unwrap();

    Ok((media_type, id))
}
