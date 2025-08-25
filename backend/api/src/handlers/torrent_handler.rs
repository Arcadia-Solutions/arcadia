use actix_multipart::form::MultipartForm;
use actix_web::{
    http::header::{
        Charset, ContentDisposition, ContentType, DispositionParam, DispositionType, ExtendedValue,
    },
    web, HttpResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::{IntoParams, ToSchema};

use crate::{handlers::User, Arcadia};
use arcadia_common::{
    error::{Error, Result},
    services::torrent_service::get_announce_url,
};
use arcadia_storage::models::torrent::{
    EditedTorrent, Torrent, TorrentMinimal, TorrentSearch, TorrentSearchResults, TorrentToDelete,
    UploadedTorrent,
};

#[utoipa::path(
    post,
    path = "/api/torrent",
    request_body(content = UploadedTorrent, content_type = "multipart/form-data"),
    responses(
        (status = 201, description = "Successfully uploaded the torrent", body=Torrent),
    )
)]
pub async fn upload_torrent(
    form: MultipartForm<UploadedTorrent>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    // TODO : check if user can upload

    let torrent = arc.pool.create_torrent(&form, &current_user).await?;

    Ok(HttpResponse::Created().json(torrent))
}

#[utoipa::path(
    put,
    path = "/api/torrent",
    responses(
        (status = 200, description = "Successfully edited the torrent", body=Torrent),
    )
)]
pub async fn edit_torrent(
    form: web::Json<EditedTorrent>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let torrent = arc.pool.find_torrent(form.id).await?;

    if torrent.created_by_id == current_user.id || current_user.class == "staff" {
        let updated_torrent = arc.pool.update_torrent(&form, torrent.id).await?;
        Ok(HttpResponse::Ok().json(updated_torrent))
    } else {
        Err(Error::InsufficientPrivileges)
    }
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct DownloadTorrentQuery {
    id: i64,
}

#[utoipa::path(
    get,
    path = "/api/torrent",
    params (DownloadTorrentQuery),
    responses(
        (status = 200, description = "Successfully downloaded the torrent file"),
    )
)]
pub async fn download_dottorrent_file(
    query: web::Query<DownloadTorrentQuery>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let torrent = arc
        .pool
        .get_torrent(
            &current_user,
            query.id,
            &arc.tracker.name,
            arc.frontend_url.as_ref(),
            arc.tracker.url.as_ref(),
        )
        .await?;

    let cd = ContentDisposition {
        disposition: DispositionType::Attachment,
        parameters: vec![
            DispositionParam::FilenameExt(ExtendedValue {
                charset: Charset::Ext(String::from("UTF-8")),
                language_tag: None,
                value: format!("{}.torrent", torrent.title).into_bytes(),
            }),
            // TODO: add fallback for better compatibility
            //DispositionParam::Filename(format!("{}.torrent", name_ascii))
        ],
    };

    Ok(HttpResponse::Ok()
        .insert_header(ContentType::octet_stream())
        .insert_header(cd)
        .body(torrent.file_contents))
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UploadInformation {
    announce_url: String,
}

#[utoipa::path(
    get,
    path = "/api/upload",
    responses(
        (status = 200, description = "Information related to uploading", body=UploadInformation),
    )
)]
pub async fn get_upload_information(
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let announce_url = get_announce_url(
        current_user.passkey_upper,
        current_user.passkey_lower,
        arc.tracker.url.as_ref(),
    );

    Ok(HttpResponse::Ok().json(UploadInformation { announce_url }))
}

#[utoipa::path(
    get,
    path = "/api/search/torrent/lite",
    responses(
        (status = 200, description = "Title groups and their torrents found", body=TorrentSearchResults),
    )
)]
pub async fn find_torrents(
    form: web::Json<TorrentSearch>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    let search_results = arc
        .pool
        .search_torrents(&form, Some(current_user.id))
        .await?;

    Ok(HttpResponse::Ok().json(search_results))
}

// #[derive(Debug, Deserialize, ToSchema)]
// pub enum SearchPeriod {
//     #[serde(rename = "24 hours")]
//     TwentyFourHours,
//     #[serde(rename = "30 days")]
//     ThirtyDays,
//     #[serde(rename = "1 year")]
//     OneYear,
//     #[serde(rename = "all time")]
//     AllTime,
// }

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct GetTopTorrentsQuery {
    period: String,
    amount: i64,
}

#[utoipa::path(
    get,
    path = "/api/torrent/top",
    params(GetTopTorrentsQuery),
    responses(
        (status = 200, description = "Top torrents found (and their title/edition group), sorted by amount of users who seeded at some point in time", body=TorrentSearchResults),
    )
)]
pub async fn get_top_torrents(
    query: web::Query<GetTopTorrentsQuery>,
    arc: web::Data<Arcadia>,
) -> Result<HttpResponse> {
    let search_results = arc
        .pool
        .find_top_torrents(&query.period, query.amount)
        .await?;

    Ok(HttpResponse::Ok().json(search_results))
}

#[utoipa::path(
    delete,
    path = "/api/torrent",
    responses(
        (status = 200, description = "Torrent deleted"),
    )
)]
pub async fn delete_torrent(
    mut form: web::Json<TorrentToDelete>,
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    if current_user.class != "staff" {
        return Err(Error::InsufficientPrivileges);
    }
    let user_url = &arc
        .frontend_url
        .join(&format!("/user/{}", current_user.id))
        .unwrap();
    let displayed_reason = format!(
        "A torrent you were a seeder on, has been deleted.
Please remove it from your torrent client.

Reason: {}

Handled by: [url={}]{}[/url]",
        &form.reason,
        &user_url.as_str(),
        current_user.username
    );

    form.displayed_reason = Some(displayed_reason);
    arc.pool.remove_torrent(&form, current_user.id).await?;

    Ok(HttpResponse::Ok().json(json!({"result": "success"})))
}

#[utoipa::path(
    get,
    path = "/api/registered-torrents",
    responses(
        (status = 200, description = "All registered torrents", body=Vec<TorrentMinimal>),
    )
)]
pub async fn get_registered_torrents(
    arc: web::Data<Arcadia>,
    current_user: User,
) -> Result<HttpResponse> {
    if current_user.class != "tracker" {
        return Err(Error::InsufficientPrivileges);
    };
    let torrents = arc.pool.find_registered_torrents().await?;

    Ok(HttpResponse::Ok().json(torrents))
}
