use actix_web::{dev::ServiceRequest, http::Method};
use arcadia_storage::models::arcadia_settings::HttpMethod;
use std::net::IpAddr;

pub mod api_key_scopes;
pub mod auth_middleware;
pub mod rate_limit;
pub mod side_effects;

/// The methods the settings and the rate limiting rules can name. Any other method (HEAD,
/// OPTIONS, ...) is not matched by a rule.
pub fn http_method(method: &Method) -> Option<HttpMethod> {
    match *method {
        Method::GET => Some(HttpMethod::Get),
        Method::POST => Some(HttpMethod::Post),
        Method::PUT => Some(HttpMethod::Put),
        Method::PATCH => Some(HttpMethod::Patch),
        Method::DELETE => Some(HttpMethod::Delete),
        _ => None,
    }
}

/// The address of the client: the configured reverse proxy header when it carries a usable
/// value, the connecting address otherwise. Reading the header without a reverse proxy
/// overwriting it would let a client forge whatever address suits it, which both mints rate
/// limiting buckets and forges the origin of a tracker request. Falling back over-limits the
/// requests that skipped the proxy, which beats exempting them from the limiter entirely.
pub fn client_ip(request: &ServiceRequest, header_name: Option<&str>) -> Option<IpAddr> {
    header_name
        .and_then(|header_name| request.headers().get(header_name))
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.rsplit(',').next())
        .and_then(|value| value.trim().parse().ok())
        .or_else(|| request.peer_addr().map(|address| address.ip()))
}
