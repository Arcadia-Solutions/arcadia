use crate::handlers::scrapers::ExternalDBData;
use actix_web::{web, HttpResponse};
use arcadia_storage::models::{
    edition_group::{create_default_edition_group, UserCreatedEditionGroup},
    title_group::{create_default_title_group, ContentType, UserCreatedTitleGroup},
};
use chrono::Utc;
// Datelike and Timelike are needed in the tests, even though they are not directly referenced
use arcadia_common::error::Result;
#[allow(unused_imports)]
use chrono::{DateTime, Datelike, NaiveDate, Timelike};
use serde::Deserialize;
use serde_json::json;
use utoipa::IntoParams;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Description {
    Typed {
        #[allow(unused)]
        #[serde(rename = "type")]
        type_: String,
        value: String,
    },
    Untyped(String),
}

impl AsRef<str> for Description {
    fn as_ref(&self) -> &str {
        match self {
            Description::Typed { value, .. } => value,
            Description::Untyped(value) => value,
        }
    }
}

impl From<Description> for String {
    fn from(d: Description) -> Self {
        match d {
            Description::Typed { value, .. } => value,
            Description::Untyped(value) => value,
        }
    }
}

#[derive(Debug, Deserialize)]
struct Work {
    title: String,
    description: Option<Description>,
    first_publish_date: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WorkLink {
    key: String,
}

#[derive(Debug, Deserialize)]
struct Book {
    works: Vec<WorkLink>,
    #[serde(rename = "covers")]
    cover_ids: Vec<u64>,
    isbn_13: Vec<String>,
}

fn parse_date(date: &str) -> Option<DateTime<Utc>> {
    date.parse::<i32>()
        .ok()
        .and_then(|y| NaiveDate::from_ymd_opt(y, 1, 1))
        .or_else(|| NaiveDate::parse_from_str(date, "%B %d, %Y").ok())
        .and_then(|nd| nd.and_hms_opt(0, 0, 0))
        .map(|ndt| DateTime::<Utc>::from_naive_utc_and_offset(ndt, *Utc::now().offset()))
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetISBNDataQuery {
    isbn: String,
}

#[utoipa::path(
    get,
    operation_id = "Get isbn data",
    tag = "External Source",
    path = "/api/external-sources/isbn",
    params(GetISBNDataQuery),
    responses(
        (status = 200, description = "", body=ExternalDBData),
    )
)]
pub async fn exec(query: web::Query<GetISBNDataQuery>) -> Result<HttpResponse> {
    let book_url = format!("https://openlibrary.org/isbn/{}.json", query.isbn);
    let book = reqwest::get(&book_url).await?.json::<Book>().await?;
    let work_path = &book.works.first().unwrap().key;
    let work_url = format!("https://openlibrary.org/{}.json", &work_path);
    let work = reqwest::get(&work_url).await?.json::<Work>().await?;

    let original_release_date = work.first_publish_date.as_ref().and_then(|d| parse_date(d));

    let description = work
        .description
        .map(String::from)
        .unwrap_or_else(|| "Description not provided.".into());

    let title_group = UserCreatedTitleGroup {
        name: work.title,
        description,
        external_links: vec![format!("https://openlibrary.org{}", work_path)],
        original_release_date,
        covers: vec![format!(
            "https://covers.openlibrary.org/b/id/{}-L.jpg",
            book.cover_ids.first().unwrap()
        )],
        content_type: ContentType::Book,
        ..create_default_title_group()
    };

    let edition_group = UserCreatedEditionGroup {
        additional_information: Some(json!({"isbn_13": book.isbn_13.first().unwrap()})),
        ..create_default_edition_group()
    };

    Ok(HttpResponse::Ok().json(ExternalDBData {
        title_group: Some(title_group),
        edition_group: Some(edition_group),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialization_moby_dick() {
        let moby_dick = serde_json::from_str::<Work>(include_str!("testdata/OL102749W.json"));

        assert!(moby_dick.is_ok());

        let moby_dick = moby_dick.unwrap();

        assert_eq!(moby_dick.title, "Moby Dick");
        assert!(moby_dick.description.is_some());
        assert_eq!(
            moby_dick.description.unwrap().as_ref(),
            "\"Command the murderous chalices! Drink ye harpooners! Drink and swear, ye men that man the deathful whaleboat's bow -- Death to Moby Dick!\" So Captain Ahab binds his crew to fulfil his obsession -- the destruction of the great white whale. Under his lordly but maniacal command the Pequod's commercial mission is perverted to one of vengeance. To Ahab, the monster that destroyed his body is not a creature, but the symbol of \"some unknown but still reasoning thing.\" Uncowed by natural disasters, ill omens, even death, Ahab urges his ship towards \"the undeliverable, nameless perils of the whale.\" Key letters from Melville to Nathaniel Hawthorne are printed at the end of this volume. - Back cover."
        );
        assert!(moby_dick.first_publish_date.is_none());
    }

    #[test]
    fn test_deserialization_les_mis() {
        let les_mis = serde_json::from_str::<Work>(include_str!("testdata/OL1063588W.json"));

        assert!(les_mis.is_ok());

        let les_mis = les_mis.unwrap();

        assert_eq!(les_mis.title, "Les Misérables");
        assert!(les_mis.description.is_some());
        assert_eq!(
            les_mis.description.unwrap().as_ref(),
            "In this story of the trials of the peasant Jean Valjean--a man unjustly imprisoned, baffled by destiny, and hounded by his nemesis, the magnificently realized, ambiguously malevolent police detective Javert--Hugo achieves the sort of rare imaginative resonance that allows a work of art to transcend its genre."
        );
        assert_eq!(les_mis.first_publish_date, Some("1863".into()));
    }

    #[test]
    fn test_parse_date() {
        // OpenLibrary published date is not normalized, try a couple varieties.

        // let local_offset_hours = Local::now().offset().local_minus_utc() / 3600;

        let date1 = parse_date("1970").unwrap();
        assert_eq!(
            (
                date1.year(),
                date1.month(),
                date1.day(),
                date1.hour(),
                date1.minute(),
                date1.second(),
            ),
            // (1970, 1, 1, local_offset_hours as u32, 0, 0)
            (1970, 1, 1, 0, 0, 0)
        );

        let date2 = parse_date("February 19, 1994").unwrap();
        assert_eq!(
            (
                date2.year(),
                date2.month(),
                date2.day(),
                date2.hour(),
                date2.minute(),
                date2.second(),
            ),
            // (1994, 2, 19, local_offset_hours as u32, 0, 0)
            (1994, 2, 19, 0, 0, 0)
        );
    }
}
