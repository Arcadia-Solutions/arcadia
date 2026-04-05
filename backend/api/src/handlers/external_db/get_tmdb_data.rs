use std::str::FromStr;

use crate::{
    handlers::scrapers::ExternalDBData, middlewares::auth_middleware::Authdata,
    services::external_db_service::check_if_existing_title_group_with_link_exists, Arcadia,
};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    connection_pool::ConnectionPool,
    models::{
        artist::{AffiliatedArtistHierarchy, ArtistRole, UserCreatedArtist},
        edition_group::{create_default_edition_group, UserCreatedEditionGroup},
        title_group::{
            create_default_title_group, ContentType, ExternalDB, PublicRating,
            UserCreatedTitleGroup,
        },
        torrent::Language,
    },
    redis::RedisPoolInterface,
};
use chrono::Utc;
use regex::Regex;
use serde::Deserialize;
use tmdb_api::client::reqwest::Client as ReqwestClient;
use tmdb_api::client::Client;
use tmdb_api::common::credits::{Cast, Crew};
use utoipa::IntoParams;

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetTMDBQuery {
    url: String,
}

fn map_crew_job_to_role(job: &str) -> Option<ArtistRole> {
    match job {
        "Director" => Some(ArtistRole::Director),
        "Producer" | "Executive Producer" => Some(ArtistRole::Producer),
        "Writer" | "Screenplay" | "Story" => Some(ArtistRole::Writer),
        "Original Music Composer" | "Music" => Some(ArtistRole::Composer),
        "Director of Photography" => Some(ArtistRole::Cinematographer),
        "Editor" => Some(ArtistRole::Editor),
        _ => None,
    }
}

fn map_crew_job_to_roles(job: &str) -> Vec<ArtistRole> {
    map_crew_job_to_role(job).into_iter().collect()
}

fn tmdb_profile_picture_url(profile_path: &Option<String>) -> Vec<String> {
    profile_path
        .as_ref()
        .map(|path| vec![format!("https://image.tmdb.org/t/p/w500{path}")])
        .unwrap_or_default()
}

async fn create_artists_from_credits(
    pool: &ConnectionPool,
    cast: &[Cast],
    crew: &[Crew],
    current_user_id: i32,
) -> Result<Vec<AffiliatedArtistHierarchy>> {
    let mut artists_to_create: Vec<UserCreatedArtist> = Vec::new();

    // Collect cast members
    for member in cast {
        artists_to_create.push(UserCreatedArtist {
            name: member.person.name.clone(),
            aliases: vec![],
            description: String::new(),
            pictures: tmdb_profile_picture_url(&member.person.profile_path),
        });
    }

    // Collect crew members
    for member in crew {
        artists_to_create.push(UserCreatedArtist {
            name: member.person.name.clone(),
            aliases: vec![],
            description: String::new(),
            pictures: tmdb_profile_picture_url(&member.person.profile_path),
        });
    }

    if artists_to_create.is_empty() {
        return Ok(vec![]);
    }

    let created_artists = pool
        .create_artists(&artists_to_create, current_user_id)
        .await?;

    let mut affiliated_artists: Vec<AffiliatedArtistHierarchy> = Vec::new();

    // Map cast to affiliated artists
    for member in cast {
        if let Some(artist) = created_artists
            .iter()
            .find(|a| a.name == member.person.name)
        {
            affiliated_artists.push(AffiliatedArtistHierarchy {
                id: 0,
                title_group_id: 0,
                artist_id: artist.id,
                roles: vec![ArtistRole::Actor],
                nickname: Some(member.character.clone()),
                created_at: Utc::now(),
                created_by_id: current_user_id,
                artist: artist.clone(),
            });
        }
    }

    // Map crew to affiliated artists
    for member in crew {
        if let Some(artist) = created_artists
            .iter()
            .find(|a| a.name == member.person.name)
        {
            let roles = map_crew_job_to_roles(&member.job);
            // Check if this artist already has an entry, and if so merge the roles
            if let Some(existing) = affiliated_artists
                .iter_mut()
                .find(|a| a.artist_id == artist.id)
            {
                for role in roles {
                    if !existing.roles.contains(&role) {
                        existing.roles.push(role);
                    }
                }
            } else {
                affiliated_artists.push(AffiliatedArtistHierarchy {
                    id: 0,
                    title_group_id: 0,
                    artist_id: artist.id,
                    roles,
                    nickname: None,
                    created_at: Utc::now(),
                    created_by_id: current_user_id,
                    artist: artist.clone(),
                });
            }
        }
    }

    Ok(affiliated_artists)
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
        original_language: Some(
            Language::from_str(&tmdb_movie.inner.original_language).unwrap_or(Language::Other),
        ),
        original_release_date: tmdb_movie.inner.release_date,
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
        release_date: title_group.original_release_date,
        ..create_default_edition_group()
    };
    Ok(ExternalDBData {
        title_group: Some(title_group),
        edition_group: Some(edition_group),
        affiliated_artists: vec![],
        existing_title_group_id: None,
    })
}

#[utoipa::path(
    get,
    operation_id = "Get TMDB data",
    tag = "External Source",
    path = "/api/external-sources/tmdb",
    params(GetTMDBQuery),
    responses(
        (status = 200, description = "", body=ExternalDBData),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<GetTMDBQuery>,
    arc: Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    if let Some(response) =
        check_if_existing_title_group_with_link_exists(&arc.pool, &query.url).await?
    {
        return Ok(response);
    }

    if arc.tmdb_api_key.is_none() {
        return Err(Error::TMDBDataFetchingNotAvailable);
    }
    let (media_type, id) = extract_media_type_and_id(&query.url).unwrap();

    let client = Client::builder()
        .with_executor(arc.http_client.clone())
        .with_api_key(arc.tmdb_api_key.clone().unwrap())
        .build()
        .expect("Failed to build TMDB client");

    let mut external_db_data = match media_type {
        ContentType::Movie => get_tmdb_movie_data(&client, id).await?,
        ContentType::TVShow => todo!(),
        // should never happen
        _ => return Err(Error::InvalidTMDBUrl),
    };

    // Fetch credits and create artists
    let credits = match media_type {
        ContentType::Movie => client
            .get_movie_credits(id, &Default::default())
            .await
            .map_err(|_| Error::TMDBDataFetchingError)?,
        _ => unreachable!(),
    };

    external_db_data.affiliated_artists =
        create_artists_from_credits(&arc.pool, &credits.cast, &credits.crew, user.sub).await?;

    if let Some(title_group) = &mut external_db_data.title_group {
        title_group.external_links.push(query.url.clone());
        crate::services::image_host_service::rehost_image_urls(
            &arc.image_host,
            &mut title_group.covers,
        )
        .await;
    }

    // Rehost artist images in the background to avoid blocking the response
    if arc.image_host.rehost_external_images {
        let image_host_config = arc.image_host.clone();
        let pool = arc.pool.clone();
        let artists: Vec<_> = external_db_data
            .affiliated_artists
            .iter()
            .map(|a| (a.artist.id, a.artist.pictures.clone()))
            .collect();

        tokio::spawn(async move {
            for (artist_id, mut pictures) in artists {
                crate::services::image_host_service::rehost_image_urls(
                    &image_host_config,
                    &mut pictures,
                )
                .await;
                if let Err(e) = pool.update_artist_pictures(artist_id, &pictures).await {
                    log::warn!("Failed to update rehosted pictures for artist {artist_id}: {e}");
                }
            }
        });
    }

    Ok(HttpResponse::Ok().json(external_db_data))
}

pub async fn get_tmdb_rating(
    http_client: &reqwest::Client,
    tmdb_url: &str,
    tmdb_api_key: String,
) -> Result<PublicRating> {
    let (media_type, id) = extract_media_type_and_id(tmdb_url).unwrap();

    let client = Client::builder()
        .with_executor(http_client.clone())
        .with_api_key(tmdb_api_key)
        .build()
        .expect("Failed to build TMDB client");

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
