use arcadia_common::error::{Error, Result};
use url::Url;

pub fn validate_image_urls(image_urls: &[String], approved_image_hosts: &[String]) -> Result<()> {
    if approved_image_hosts.is_empty() {
        return Ok(());
    }

    for image_url in image_urls {
        validate_image_url(image_url, approved_image_hosts)?;
    }
    Ok(())
}

pub fn validate_image_url(image_url: &str, approved_image_hosts: &[String]) -> Result<()> {
    if approved_image_hosts.is_empty() {
        return Ok(());
    }

    let parsed_url =
        Url::parse(image_url).map_err(|_| Error::ImageHostNotApproved(image_url.to_string()))?;

    let image_host = parsed_url
        .host_str()
        .ok_or_else(|| Error::ImageHostNotApproved(image_url.to_string()))?;

    if approved_image_hosts.iter().any(|h| h == image_host) {
        return Ok(());
    }

    Err(Error::ImageHostNotApproved(image_url.to_string()))
}
