use crate::{
    handlers::scrapers::ExternalDBData, services::common_service::naive_date_to_utc_midnight,
    Arcadia,
};
use actix_web::{
    web::{Data, Query},
    HttpResponse,
};
use arcadia_common::error::{Error, Result};
use arcadia_storage::{
    models::{
        edition_group::{create_default_edition_group, UserCreatedEditionGroup},
        title_group::{create_default_title_group, ContentType, UserCreatedTitleGroup},
    },
    redis::RedisPoolInterface,
};
use chrono::NaiveDate;
use musicbrainz_rs::{
    client::MusicBrainzClient,
    entity::{
        release::Release,
        release_group::{ReleaseGroup, ReleaseGroupPrimaryType},
        CoverartResponse,
    },
    Fetch, FetchCoverart,
};
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use utoipa::IntoParams;

async fn get_musicbrainz_release_group_data(
    id: &str,
    client: &MusicBrainzClient,
) -> Result<UserCreatedTitleGroup> {
    // TODO: handle artists
    let musicbrainz_title_group = ReleaseGroup::fetch()
        .id(id)
        .with_tags()
        .with_aliases()
        // .with_artists()
        .execute_with_client(client)
        .await
        .map_err(Error::ErrorGettingMusicbrainzData)?;
    let cover = ReleaseGroup::fetch_coverart()
        .id(id)
        .execute_with_client(client)
        .await
        .map_or_else(
            |_| String::new(),
            |c| match c {
                CoverartResponse::Url(url) => url,
                CoverartResponse::Json(coverart) => coverart
                    .images
                    .first()
                    .map_or_else(String::new, |img| img.image.clone()),
            },
        );

    Ok(UserCreatedTitleGroup {
        name: musicbrainz_title_group.title,
        name_aliases: musicbrainz_title_group
            .aliases
            .unwrap()
            .iter()
            .map(|alias| alias.name.clone())
            .collect(),
        tags: musicbrainz_title_group
            .tags
            .unwrap()
            .iter()
            .map(|tag| tag.name.clone().replace(" ", "."))
            .collect(),
        original_release_date: Some(
            musicbrainz_title_group
                .first_release_date
                .map(naive_date_to_utc_midnight)
                .unwrap(),
        ),
        category: Some(
            musicbrainz_title_group
                .primary_type
                .unwrap_or(ReleaseGroupPrimaryType::UnrecognizedReleaseGroupPrimaryType)
                .into(),
        ),
        content_type: ContentType::Music,
        external_links: vec![format!("https://musicbrainz.org/release-group/{}", id)],
        covers: vec![cover],
        ..create_default_title_group()
    })
}

async fn get_musicbrainz_release_data(
    id: &str,
    client: &MusicBrainzClient,
) -> Result<(UserCreatedEditionGroup, Option<String>)> {
    let musicbrainz_edition_group = Release::fetch()
        .id(id)
        .with_release_groups()
        .with_labels()
        .execute_with_client(client)
        .await
        .map_err(Error::ErrorGettingMusicbrainzData)?;
    Ok((
        UserCreatedEditionGroup {
            additional_information: Some(json!({
                "catalogue_number":  musicbrainz_edition_group.label_info.as_ref().and_then(|li| li.first()).and_then(|li_item| li_item.catalog_number.clone()).unwrap_or_default(), //musicbrainz_edition_group.barcode.clone().unwrap_or("".to_string()),
                "label": musicbrainz_edition_group.label_info.as_ref().and_then(|li| li.first()).and_then(|li_item| li_item.label.as_ref()).map(|label| label.name.clone()).unwrap_or_default()
            })),
            release_date: musicbrainz_edition_group
                .date
                .map(naive_date_to_utc_midnight)
                .unwrap_or_else(|| {
                    naive_date_to_utc_midnight(NaiveDate::from_ymd_opt(1900, 1, 1).unwrap())
                }),
            external_links: vec![format!("https://musicbrainz.org/release/{}", id)],
            ..create_default_edition_group()
        },
        match musicbrainz_edition_group.release_group {
            Some(release_group) => Some(release_group.id),
            _ => None,
        },
    ))
}

#[derive(Debug, PartialEq)]
pub enum MusicBrainzResourceType {
    Release,
    ReleaseGroup,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetMusicbrainzQuery {
    url: String,
}

#[utoipa::path(
    get,
    operation_id = "Get Musicbranz data",
    tag = "External Source",
    path = "/api/external-sources/musicbrainz",
    params(GetMusicbrainzQuery),
    responses(
        (status = 200, description = "", body=ExternalDBData),
    )
)]
pub async fn exec<R: RedisPoolInterface + 'static>(
    query: Query<GetMusicbrainzQuery>,
    arc: Data<Arcadia<R>>,
) -> Result<HttpResponse> {
    let (entity_type, id) = Regex::new(r"musicbrainz.org/(release|release-group)/([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})")
        .expect("Regex error")
        .captures(&query.url).map(|caps| (match caps[1].as_ref() { "release" => MusicBrainzResourceType::Release, _ => MusicBrainzResourceType::ReleaseGroup }, caps[2].to_string()))
        .ok_or_else(|| Error::InvalidMusicbrainzUrl)?;
    // .expect("No MusicBrainz release/release-group match found in URL");
    let mut client = MusicBrainzClient::default();
    client
        .set_user_agent(&format!("{} ({})", arc.tracker.name, arc.frontend_url))
        .map_err(|_| Error::InvalidMusicbrainzUrl)?;

    let mut title_group: Option<UserCreatedTitleGroup> = None;
    let mut edition_group: Option<UserCreatedEditionGroup> = None;
    match entity_type {
        MusicBrainzResourceType::ReleaseGroup => {
            title_group = Some(get_musicbrainz_release_group_data(&id, &client).await?);
        }
        MusicBrainzResourceType::Release => {
            let (eg, release_group_id) = get_musicbrainz_release_data(&id, &client).await?;
            edition_group = Some(eg);
            if let Some(rgid) = release_group_id {
                title_group = Some(get_musicbrainz_release_group_data(&rgid, &client).await?);
            }
        }
    };

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "title_group": title_group,
        "edition_group": edition_group
    })))
}
