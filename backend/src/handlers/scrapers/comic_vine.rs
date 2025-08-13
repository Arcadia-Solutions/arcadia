use actix_web::{HttpResponse, web};
use chrono::{NaiveDate, Utc};
use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use std::env;
use utoipa::IntoParams;

use crate::{
    Arcadia, Error, Result,
    handlers::scrapers::ExternalDBData,
    models::title_group::{
        ContentType, TitleGroupCategory, UserCreatedTitleGroup, create_default_title_group,
    },
    services::common_service::naive_date_to_utc_midnight,
};

#[derive(Debug, Deserialize)]
struct ComicVineResponse<T> {
    pub results: T,
    // pub error: String,
}

#[derive(Debug, Deserialize)]
struct Image {
    // pub icon_url: Option<String>,
    // pub medium_url: Option<String>,
    pub original_url: Option<String>,
    // pub screen_url: Option<String>,
    // pub small_url: Option<String>,
    // pub super_url: Option<String>,
    // pub thumb_url: Option<String>,
    // pub tiny_url: Option<String>,
}

// #[derive(Debug, Deserialize)]
// struct ComicVinePublisher {
//     pub name: String,
//     pub api_detail_url: Option<String>,
//     pub site_detail_url: Option<String>,
//     pub id: i64,
// }

// #[derive(Debug, Deserialize)]
// struct ComicVineVolume {
//     pub id: i64,
//     pub name: String,
//     pub description: Option<String>,
//     pub image: Option<Image>,
//     pub publisher: Option<ComicVinePublisher>,
//     pub start_year: Option<String>,
//     pub site_detail_url: Option<String>,
// }

#[derive(Debug, Deserialize)]
struct ComicVineIssue {
    // pub id: i64,
    pub name: Option<String>,
    pub issue_number: Option<String>,
    pub cover_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub image: Option<Image>,
    // pub volume: Option<ComicVineVolume>,
    pub site_detail_url: Option<String>,
}

const COMICVINE_API_BASE_URL: &str = "https://comicvine.gamespot.com/api";

async fn fetch_comic_vine_data<T: for<'de> Deserialize<'de>>(
    endpoint: &str,
    client: &Client,
) -> Result<T> {
    let api_key = env::var("COMIC_VINCE_API_KEY").ok().unwrap();

    let url = format!("{COMICVINE_API_BASE_URL}/{endpoint}/?api_key={api_key}&format=json");
    let response = client.get(&url).send().await;
    println!("{response:?}");

    let response = client
        .get(&url)
        .send()
        .await?
        .json::<ComicVineResponse<T>>()
        .await?;

    // if response.error == "OK" {
    Ok(response.results)
    // } else {
    //     Err(())
    // }
}

async fn get_comic_vine_issue_data(id: &str, client: &Client) -> Result<UserCreatedTitleGroup> {
    let comic_vine_issue: ComicVineIssue =
        fetch_comic_vine_data(&format!("issue/4000-{id}"), client).await?;

    let cover_url = comic_vine_issue
        .image
        .and_then(|img| img.original_url)
        .unwrap_or_default();
    let issue_name = comic_vine_issue.name.unwrap_or_else(|| {
        format!(
            "Issue {}",
            comic_vine_issue.issue_number.as_deref().unwrap_or("N/A")
        )
    });

    let title_group = UserCreatedTitleGroup {
        name: issue_name,
        original_release_date: Some(
            comic_vine_issue
                .cover_date
                .map_or_else(Utc::now, naive_date_to_utc_midnight),
        ),
        content_type: ContentType::Book,
        category: Some(TitleGroupCategory::Illustrated),
        description: comic_vine_issue.description.unwrap_or("".to_string()),
        // distributor: comic_vine_issue
        //     .volume
        //     .as_ref()
        //     .and_then(|v| v.publisher.as_ref())
        //     .map(|p| p.name.clone()),
        covers: if cover_url.is_empty() {
            vec![]
        } else {
            vec![cover_url]
        },
        external_links: comic_vine_issue
            .site_detail_url
            .map_or_else(Vec::new, |url| vec![url]),
        // additional_information: Some(json!({
        //     "issue_number": comic_vine_issue.issue_number,
        // })),
        ..create_default_title_group()
    };

    Ok(title_group)
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetComicVineQuery {
    url: String,
}

#[derive(Debug, PartialEq)]
pub enum ComicVineResourceType {
    Issue,
    Volume,
}

#[utoipa::path(
    post,
    params(GetComicVineQuery),
    path = "/api/external_db/comic_vine",
    responses(
        (status = 200, description = "", body=ExternalDBData),
    )
)]
pub async fn get_comic_vine_data(
    query: web::Query<GetComicVineQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    // TODO: add contact email from config
    let client = Client::builder()
        .user_agent(format!(
            "{} ({} {})",
            arc.tracker_name, arc.frontend_url, "contact@example.com"
        ))
        .build()
        .expect("Failed to build reqwest client");
    let (entity_type, id) = Regex::new(r"comicvine.gamespot.com/.*?/(40(00|50))-([0-9]+)/?$")
        .expect("Regex error for Comic Vine URL")
        .captures(&query.url)
        .map(|caps| {
            (
                match &caps[1] {
                    "4000" => ComicVineResourceType::Issue,
                    "4050" => ComicVineResourceType::Volume,
                    _ => unreachable!(),
                },
                caps[3].to_string(),
            )
        })
        .ok_or_else(|| Error::InvalidComicVineUrl)?;

    let mut title_group: Option<UserCreatedTitleGroup> = None;
    match entity_type {
        ComicVineResourceType::Issue => {
            title_group = Some(get_comic_vine_issue_data(&id, &client).await?);
        }
        ComicVineResourceType::Volume => {}
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "title_group": title_group,
        "edition_group": null
    })))
}
