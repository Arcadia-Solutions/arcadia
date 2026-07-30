use std::time::Duration;

use crate::{
    handlers::scrapers::{ExternalDBData, ScrapedExternalData},
    middlewares::auth_middleware::Authdata,
    services::external_db_service::{
        check_if_existing_title_group_with_link_exists, create_affiliated_artists,
        respond_with_scraped_data,
    },
    Arcadia,
};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::{Error, Result};
use arcadia_storage::redis::RedisPoolInterface;

pub async fn exec<R: RedisPoolInterface + 'static>(
    source_id: &str,
    url: &str,
    arc: &Data<Arcadia<R>>,
    user: Authdata,
) -> Result<HttpResponse> {
    let plugin = arc
        .scrapers
        .iter()
        .find(|plugin| plugin.source.id == source_id)
        .ok_or_else(|| Error::ExternalSourceNotFound(source_id.to_string()))?;

    if let Some(response) = check_if_existing_title_group_with_link_exists(&arc.pool, url).await? {
        return Ok(response);
    }

    // the error the plugin failed with is only logged, as it is of no use to the uploader
    let plugin_error = |error: reqwest::Error| {
        log::warn!(
            "external source plugin '{}' ({}) failed: {error}",
            plugin.source.id,
            plugin.url
        );
        Error::ExternalSourcePluginError(plugin.source.id.clone())
    };

    // plugins run alongside arcadia, so their requests must never go through the outbound proxy
    let scraped_data = arc
        .internal_http_client
        .get(&plugin.url)
        .query(&[("url", url)])
        .timeout(Duration::from_secs(plugin.timeout_seconds))
        .send()
        .await
        .map_err(&plugin_error)?
        .error_for_status()
        .map_err(&plugin_error)?
        .json::<ScrapedExternalData>()
        .await
        .map_err(&plugin_error)?;

    let mut external_db_data = ExternalDBData {
        title_group: scraped_data.title_group,
        edition_group: scraped_data.edition_group,
        affiliated_artists: create_affiliated_artists(
            &arc.pool,
            scraped_data.affiliated_artists,
            user.sub,
        )
        .await?,
        existing_title_group_id: None,
    };

    if let Some(title_group) = &mut external_db_data.title_group {
        title_group.external_links.push(url.to_string());
    }

    respond_with_scraped_data(&arc.pool, &arc.image_host, external_db_data).await
}
