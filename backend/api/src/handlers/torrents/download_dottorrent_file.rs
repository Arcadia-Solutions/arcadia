use actix_web::{
    http::header::{
        Charset, ContentDisposition, ContentType, DispositionParam, DispositionType, ExtendedValue,
    },
    web, HttpResponse,
};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::{handlers::User, Arcadia};
use arcadia_common::error::Result;

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct DownloadTorrentQuery {
    id: i64,
}

#[utoipa::path(
    get,
    operation_id = "Download .torrent file",
    tag = "Torrent",
    path = "/api/torrents",
    params (DownloadTorrentQuery),
    responses(
        (status = 200, description = "Successfully downloaded the torrent file"),
    )
)]
pub async fn exec(
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
