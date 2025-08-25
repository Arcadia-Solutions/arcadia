use std::vec;

use crate::{
    Result,
    handlers::scrapers::ExternalDBData,
    models::title_group::{ContentType, UserCreatedTitleGroup, create_default_title_group},
};
use actix_web::{HttpResponse, web};
use serde::{Deserialize, de::DeserializeOwned};

#[derive(Debug, Deserialize)]
struct Movie {
    //id: u32,
    title: String,
    overview: String,
    release_date: String,
    //vote_average: f64,
    //vote_count: u32,
    original_language: String,
    original_title: String,
    //genres: Vec<Genre>,
    poster_path: Option<String>,
    //backdrop_path: Option<String>,
    //production_companies: Vec<ProductionCompany>,
    //runtime: Option<u32>,
    //status: String,
}

#[derive(Debug, Deserialize)]
struct TvShow {
    //id: u64,
    name: String,
    overview: String,
    first_air_date: String,
    //last_air_date: String,
    //number_of_seasons: u32,
    //number_of_episodes: u32,
    //status: String,
    //genres: Vec<Genre>,
    homepage: Option<String>,
    //backdrop_path: Option<String>,
    poster_path: Option<String>,
    //vote_average: f32,
    //vote_count: u32,
}

//#[derive(Debug, Deserialize)]
//struct Genre {
//    id: u32,
//    name: String,
//}

//#[derive(Debug, Deserialize)]
//struct ProductionCompany {
//    id: u32,
//    name: String,
//    logo_path: Option<String>,
//    origin_country: String,
//}

#[derive(Debug, Deserialize)]
struct Configuration {
    images: Images,
}

#[derive(Debug, Deserialize)]
struct Images {
    base_url: String,
    //secure_base_url: String,
    poster_sizes: Vec<String>,
    //backdrop_sizes: Vec<String>,
    //profile_sizes: Vec<String>,
    //logo_sizes: Vec<String>,
    //still_sizes: Vec<String>,
}

struct Tmdb {
    client: reqwest::Client,
    bearer: String,
    config: Configuration,
}

const BASE_URL: &str = "https://api.themoviedb.org/3";

impl Tmdb {
    async fn new(bearer: impl Into<String>) -> Result<Self> {
        let client = reqwest::Client::new();
        let bearer = bearer.into();

        let config = client
            .get(format!("{}/configuration", BASE_URL))
            .bearer_auth(&bearer)
            .send()
            .await?
            .json::<Configuration>()
            .await?;

        Ok(Tmdb {
            client,
            bearer,
            config,
        })
    }

    async fn call_endpoint<T: DeserializeOwned>(&self, endpoint: impl AsRef<str>) -> Result<T> {
        let resp = self
            .client
            .get(endpoint.as_ref())
            .bearer_auth(&self.bearer)
            .send()
            .await?
            .json::<T>()
            .await?;

        Ok(resp)
    }

    async fn movie_by_id(&self, id: u32) -> Result<Movie> {
        self.call_endpoint::<Movie>(format!("{}/movie/{}", BASE_URL, id))
            .await
    }

    async fn tvshow_by_id(&self, id: u32) -> Result<TvShow> {
        self.call_endpoint::<TvShow>(format!("{}/tv/{}", BASE_URL, id))
            .await
    }

    fn get_image_urls(&self, path: impl Into<String>) -> impl Iterator<Item = String> {
        let path = path.into();
        self.config
            .images
            .poster_sizes
            .iter()
            .map(move |sz| format!("{}{}{}", self.config.images.base_url, sz, path))
    }
}

#[derive(Debug, Deserialize)]
pub struct TmdbQuery {
    id: u32,
}

#[utoipa::path(
    post,
    path = "/api/external_db/tmdb/movie",
    responses(
        (status = 200, description = "", body=ExternalDBData),
    )
)]
pub async fn get_tmdb_movie_data(query: web::Query<TmdbQuery>) -> Result<HttpResponse> {
    let token = std::env::var("TMDB_API_KEY").expect("TMDB_API_KEY must be set");

    // TODO: create this only once.
    let tmdb = Tmdb::new(token).await?;

    let movie = tmdb.movie_by_id(query.id).await?;

    let name_aliases = if movie.title != movie.original_title {
        vec![movie.original_title]
    } else {
        vec![]
    };

    let covers = movie
        .poster_path
        .as_ref()
        .map(|p| tmdb.get_image_urls(p).collect())
        .unwrap_or(vec![]);

    let original_release_date = chrono::NaiveDate::parse_from_str(&movie.release_date, "%Y-%m-%d")
        .ok()
        .and_then(|nd| nd.and_hms_opt(0, 0, 0))
        .map(|ndt| {
            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                ndt,
                *chrono::Utc::now().offset(),
            )
        })
        .unwrap();

    let title_group = UserCreatedTitleGroup {
        name: movie.title,
        name_aliases,
        description: movie.overview,
        original_language: Some(movie.original_language),
        covers,
        external_links: vec![format!("https://www.themoviedb.org/movie/{}", query.id)],
        content_type: ContentType::Movie,
        original_release_date,
        ..create_default_title_group()
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({ "title_group": title_group })))
}

#[utoipa::path(
    post,
    path = "/api/external_db/tmdb/tv",
    responses(
        (status = 200, description = "", body=ExternalDBData),
    )
)]
pub async fn get_tmdb_tv_data(query: web::Query<TmdbQuery>) -> Result<HttpResponse> {
    let token = std::env::var("TMDB_API_KEY").expect("TMDB_API_KEY must be set");

    // TODO: create this only once.
    let tmdb = Tmdb::new(token).await?;

    let tvshow = tmdb.tvshow_by_id(query.id).await?;

    let covers = tvshow
        .poster_path
        .as_ref()
        .map(|p| tmdb.get_image_urls(p).collect())
        .unwrap_or(vec![]);

    let mut external_links = vec![format!("https://www.themoviedb.org/tv/{}", query.id)];

    if let Some(homepage) = tvshow.homepage {
        external_links.push(homepage);
    }

    let original_release_date =
        chrono::NaiveDate::parse_from_str(&tvshow.first_air_date, "%Y-%m-%d")
            .ok()
            .and_then(|nd| nd.and_hms_opt(0, 0, 0))
            .map(|ndt| {
                chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(
                    ndt,
                    *chrono::Utc::now().offset(),
                )
            })
            .unwrap();

    let title_group = UserCreatedTitleGroup {
        name: tvshow.name,
        description: tvshow.overview,
        covers,
        external_links: vec![format!("https://www.themoviedb.org/tv/{}", query.id)],
        content_type: ContentType::TVShow,
        original_release_date,
        ..create_default_title_group()
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({ "title_group": title_group })))
}
