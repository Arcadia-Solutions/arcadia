use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::uri::Uri,
    middleware::Next,
    web::Data,
};
use arcadia_storage::redis::RedisPoolInterface;
use url::form_urlencoded;

use crate::Arcadia;

const PAGE_SIZE_PARAMETER: &str = "page_size";

/// Caps the `page_size` query parameter of every request to `api.max_page_size`, so that a client
/// cannot ask a paginated endpoint for an arbitrarily big page.
pub async fn cap_page_size_middleware<R: RedisPoolInterface + 'static>(
    mut req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let max_page_size = req
        .app_data::<Data<Arcadia<R>>>()
        .map(|arcadia| arcadia.api.max_page_size);

    if let Some(max_page_size) = max_page_size
        && let Some(capped_query_string) = cap_query_string(req.query_string(), max_page_size)
    {
        let path = req.uri().path().to_owned();
        let mut uri_parts = req.uri().clone().into_parts();
        uri_parts.path_and_query = Some(
            format!("{path}?{capped_query_string}")
                .parse()
                .map_err(actix_web::error::ErrorInternalServerError)?,
        );
        req.head_mut().uri =
            Uri::from_parts(uri_parts).map_err(actix_web::error::ErrorInternalServerError)?;
    }

    next.call(req).await
}

/// Returns the query string with its `page_size` parameters capped, or `None` when nothing had to
/// be capped. Every other parameter, and a `page_size` that is not a number, are left untouched so
/// that the handler keeps rejecting them.
fn cap_query_string(query_string: &str, max_page_size: u32) -> Option<String> {
    let mut capped_something = false;

    let capped_query_string = query_string
        .split('&')
        .map(|parameter| {
            if !is_page_size_over_the_maximum(parameter, max_page_size) {
                return parameter.to_owned();
            }
            capped_something = true;
            format!("{PAGE_SIZE_PARAMETER}={max_page_size}")
        })
        .collect::<Vec<_>>()
        .join("&");

    capped_something.then_some(capped_query_string)
}

/// Whether the given `name=value` pair is a `page_size` bigger than the maximum. The pair is
/// decoded the same way the handlers decode it when deserializing the query string (percent
/// decoding, and `+` as a space), so that an encoded parameter such as `page%5Fsize=9999` or
/// `page_size=%399999` cannot slip past the cap.
fn is_page_size_over_the_maximum(parameter: &str, max_page_size: u32) -> bool {
    let Some((name, value)) = form_urlencoded::parse(parameter.as_bytes()).next() else {
        return false;
    };

    name == PAGE_SIZE_PARAMETER
        && value
            .parse::<u64>()
            .is_ok_and(|page_size| page_size > max_page_size as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX_PAGE_SIZE: u32 = 100;

    fn cap(query_string: &str) -> Option<String> {
        cap_query_string(query_string, MAX_PAGE_SIZE)
    }

    #[test]
    fn a_page_size_up_to_the_maximum_is_left_untouched() {
        assert_eq!(cap(""), None);
        assert_eq!(cap("page=1"), None);
        assert_eq!(cap("page_size=1&page=2"), None);
        assert_eq!(cap("page_size=100"), None);
    }

    #[test]
    fn a_page_size_over_the_maximum_is_capped() {
        assert_eq!(cap("page_size=101"), Some("page_size=100".to_owned()));
        assert_eq!(
            cap("page=2&page_size=9999&order_by_column=name"),
            Some("page=2&page_size=100&order_by_column=name".to_owned())
        );
    }

    #[test]
    fn every_page_size_of_the_query_string_is_capped() {
        assert_eq!(
            cap("page_size=9999&page_size=8888"),
            Some("page_size=100&page_size=100".to_owned())
        );
    }

    #[test]
    fn an_encoded_page_size_is_capped_too() {
        // the handlers percent decode the query string, so the cap has to decode it as well
        assert_eq!(cap("page%5Fsize=9999"), Some("page_size=100".to_owned()));
        assert_eq!(cap("page_size=%399999"), Some("page_size=100".to_owned()));
        assert_eq!(cap("page_size=%2B9999"), Some("page_size=100".to_owned()));
        // `+` is a space and `%20` is a space, so those are not numbers for the handler either
        assert_eq!(cap("page_size=+9999"), None);
        assert_eq!(cap("page_size=9999%20"), None);
    }

    #[test]
    fn a_page_size_that_is_not_a_number_is_left_to_the_handler() {
        assert_eq!(cap("page_size"), None);
        assert_eq!(cap("page_size="), None);
        assert_eq!(cap("page_size=-1"), None);
        assert_eq!(cap("page_size=abc"), None);
        assert_eq!(cap("page_size=99999999999999999999999999"), None);
    }

    #[test]
    fn a_parameter_that_merely_looks_like_a_page_size_is_left_untouched() {
        assert_eq!(cap("not_page_size=9999"), None);
        assert_eq!(cap("page_size_of_something=9999"), None);
        assert_eq!(cap("name=page_size=9999"), None);
    }
}
