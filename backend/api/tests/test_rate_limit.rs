pub mod common;
pub mod mocks;

use crate::common::{auth_header, create_test_app_with_rate_limits, login_as, TestUser};
use crate::mocks::mock_redis::MockRedisPool;
use actix_http::Request;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::http::{Method, StatusCode};
use actix_web::test::{call_service, TestRequest};
use actix_web::Error;
use arcadia_api::config::RateLimitsConfig;
use arcadia_api::middlewares::client_ip;
use arcadia_api::middlewares::rate_limit::{
    mask_ipv6_client_address, RateLimitDecision, RateLimitPolicy, RateLimitSubject,
};
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::models::arcadia_settings::HttpMethod;
use sqlx::PgPool;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;

fn rate_limits(configuration: &str) -> RateLimitsConfig {
    serde_norway::from_str(configuration).expect("the configuration should be parsed")
}

fn test_policy() -> RateLimitPolicy {
    RateLimitPolicy::new(&rate_limits(
        r#"
default:
  requests: 10
  per_seconds: 60
rules:
  - path_prefix: /api/auth/login
    method: POST
    requests: 1
    per_seconds: 60
  - path_prefix: /api/auth
    requests: 2
    per_seconds: 60
"#,
    ))
}

fn test_subject() -> RateLimitSubject {
    RateLimitSubject::Ip(IpAddr::V4(Ipv4Addr::new(10, 10, 4, 88)))
}

#[test]
fn test_the_first_matching_rule_wins() {
    let policy = test_policy();
    let subject = test_subject();

    assert!(matches!(
        policy.check(&subject, "/api/auth/login", Some(&HttpMethod::Post)),
        RateLimitDecision::Allowed
    ));

    match policy.check(&subject, "/api/auth/login", Some(&HttpMethod::Post)) {
        RateLimitDecision::Rejected {
            rule,
            retry_after_seconds,
        } => {
            assert_eq!(rule, "/api/auth/login");
            // The rule's window is 60 seconds and the single burst slot was just consumed, so
            // the wait is essentially the full window; allow a few seconds of test slack rather
            // than asserting equality against a real clock.
            assert!(retry_after_seconds > 55 && retry_after_seconds <= 60);
        }
        RateLimitDecision::Allowed => panic!("the second request should be rejected"),
    }
}

#[test]
fn test_a_rule_without_method_matches_every_method() {
    let policy = test_policy();
    let subject = test_subject();

    // The second rule names no method, so both of these fall on its quota of two.
    assert!(matches!(
        policy.check(&subject, "/api/auth/refresh", Some(&HttpMethod::Post)),
        RateLimitDecision::Allowed
    ));
    assert!(matches!(
        policy.check(&subject, "/api/auth/refresh", Some(&HttpMethod::Get)),
        RateLimitDecision::Allowed
    ));

    match policy.check(&subject, "/api/auth/refresh", Some(&HttpMethod::Get)) {
        RateLimitDecision::Rejected { rule, .. } => assert_eq!(rule, "/api/auth"),
        RateLimitDecision::Allowed => panic!("the third request should be rejected"),
    }
}

#[test]
fn test_the_default_quota_applies_when_no_rule_matches() {
    let policy = test_policy();
    let subject = test_subject();

    for _ in 0..10 {
        assert!(matches!(
            policy.check(&subject, "/api/users/me", Some(&HttpMethod::Get)),
            RateLimitDecision::Allowed
        ));
    }

    match policy.check(&subject, "/api/users/me", Some(&HttpMethod::Get)) {
        RateLimitDecision::Rejected { rule, .. } => assert_eq!(rule, "default"),
        RateLimitDecision::Allowed => panic!("the eleventh request should be rejected"),
    }
}

#[test]
fn test_every_subject_has_its_own_bucket() {
    let policy = test_policy();
    let first_subject = test_subject();
    let second_subject = RateLimitSubject::Ip(IpAddr::V4(Ipv4Addr::new(10, 10, 4, 89)));
    let third_subject = RateLimitSubject::User(1);

    for subject in [&first_subject, &second_subject, &third_subject] {
        assert!(matches!(
            policy.check(subject, "/api/auth/login", Some(&HttpMethod::Post)),
            RateLimitDecision::Allowed
        ));
    }

    assert!(matches!(
        policy.check(&first_subject, "/api/auth/login", Some(&HttpMethod::Post)),
        RateLimitDecision::Rejected { .. }
    ));
}

#[test]
fn test_a_rule_matching_no_endpoint_is_reported() {
    let endpoint_paths = [
        "/api/auth/login".to_owned(),
        "/api/torrents".to_owned(),
        "/api/users/{id}".to_owned(),
    ];
    let policy = RateLimitPolicy::new(&rate_limits(
        r#"
default:
  requests: 10
  per_seconds: 60
rules:
  - path_prefix: /api/auth/login
    requests: 1
    per_seconds: 60
  - path_prefix: /api/torrent
    requests: 1
    per_seconds: 60
  - path_prefix: /api/users/{id}
    requests: 1
    per_seconds: 60
"#,
    ));

    // "/api/torrent" is a prefix of the "/api/torrents" endpoint, so it does match something,
    // and a path holding a parameter is compared as the openapi spec writes it.
    assert_eq!(
        policy.rules_matching_no_endpoint(&endpoint_paths),
        Vec::<&str>::new()
    );

    let policy = RateLimitPolicy::new(&rate_limits(
        r#"
default:
  requests: 10
  per_seconds: 60
rules:
  - path_prefix: /api/auth/login
    requests: 1
    per_seconds: 60
  - path_prefix: /api/torrrents
    requests: 1
    per_seconds: 60
"#,
    ));

    assert_eq!(
        policy.rules_matching_no_endpoint(&endpoint_paths),
        vec!["/api/torrrents"],
        "a typo in a rule prefix has to be reported"
    );
}

#[test]
fn test_the_rate_limit_error_answers_429_with_a_retry_after_header() {
    use actix_web::ResponseError;

    let error = arcadia_common::error::Error::RateLimitExceeded {
        retry_after_seconds: 42,
    };

    let response = error.error_response();

    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(
        response
            .headers()
            .get("Retry-After")
            .expect("the response should carry a Retry-After header"),
        "42"
    );
}

/// The address `client_ip` resolves for a request carrying the given `X-Forwarded-For` header
/// (when set) and peer address (when set), with that header configured.
fn resolved_client_ip(forwarded_for: Option<&str>, peer_addr: Option<&str>) -> Option<IpAddr> {
    let mut request = TestRequest::default();
    if let Some(forwarded_for) = forwarded_for {
        request = request.insert_header(("X-Forwarded-For", forwarded_for));
    }
    if let Some(peer_addr) = peer_addr {
        request = request.peer_addr(peer_addr.parse().expect("a valid socket address"));
    }

    client_ip(&request.to_srv_request(), Some("X-Forwarded-For"))
}

#[test]
fn test_the_client_ip_comes_from_the_configured_header() {
    // The header wins over the connection, and the last value of the list is the one the
    // reverse proxy appended.
    assert_eq!(
        resolved_client_ip(Some("10.10.4.88"), Some("10.10.4.1:1111")),
        Some("10.10.4.88".parse().expect("a valid address"))
    );
    assert_eq!(
        resolved_client_ip(Some("203.0.113.5, 10.10.4.88"), None),
        Some("10.10.4.88".parse().expect("a valid address"))
    );
}

#[test]
fn test_the_client_ip_falls_back_to_the_peer_address() {
    // A missing or unusable header falls back to the connection rather than leaving the client
    // unidentified, which would exempt it from the limiter.
    for forwarded_for in [None, Some("not-an-ip-address")] {
        assert_eq!(
            resolved_client_ip(forwarded_for, Some("10.10.4.3:3333")),
            Some("10.10.4.3".parse().expect("a valid address"))
        );
    }

    assert_eq!(resolved_client_ip(None, None), None);
}

#[test]
fn test_ipv6_addresses_are_masked_to_their_slash_64() {
    let masked =
        |address: &str| mask_ipv6_client_address(address.parse().expect("a valid address"));

    assert_eq!(
        masked("2001:db8:1234:5678::1"),
        masked("2001:db8:1234:5678::2")
    );
    assert_ne!(
        masked("2001:db8:1234:5678::1"),
        masked("2001:db8:aaaa:bbbb::1")
    );

    // An IPv4 address, mapped or not, keeps its full value.
    assert_ne!(masked("::ffff:10.10.4.88"), masked("::ffff:10.10.4.89"));
    assert_eq!(
        masked("10.10.4.88"),
        "10.10.4.88".parse::<IpAddr>().expect("a valid address")
    );
}

/// `requests` allowed on `POST /api/auth/login`, generous everywhere else: the low quota turns
/// "does this request share a bucket with the previous one" into a status code assertion.
fn login_rate_limits(requests: u32) -> RateLimitsConfig {
    rate_limits(&format!(
        r#"
default:
  requests: 100
  per_seconds: 60
rules:
  - path_prefix: /api/auth/login
    method: POST
    requests: {requests}
    per_seconds: 60
"#
    ))
}

/// The test requests carry no peer address, so they identify their client with the
/// `X-Forwarded-For` header `create_test_app_with_rate_limits` configures.
async fn rate_limited_service(
    pool: PgPool,
    rate_limits: RateLimitsConfig,
) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    create_test_app_with_rate_limits(
        Arc::new(ConnectionPool::with_pg_pool(pool)),
        MockRedisPool::default(),
        rate_limits,
    )
    .await
}

fn login_request(forwarded_for: &str) -> Request {
    TestRequest::post()
        .uri("/api/auth/login")
        .insert_header(("X-Forwarded-For", forwarded_for))
        .set_json(TestUser::Standard.get_login_payload())
        .to_request()
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_anonymous_requests_are_limited_per_ip(pool: PgPool) {
    let service = rate_limited_service(pool, login_rate_limits(2)).await;

    for _ in 0..2 {
        let response = call_service(&service, login_request("10.10.4.88")).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    let response = call_service(&service, login_request("10.10.4.88")).await;

    assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);
    assert!(
        response.headers().contains_key("Retry-After"),
        "a rejected request should tell the client when to retry"
    );
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_two_ip_addresses_have_separate_buckets(pool: PgPool) {
    let service = rate_limited_service(pool, login_rate_limits(1)).await;

    for ip_address in ["10.10.4.88", "10.10.4.89"] {
        let response = call_service(&service, login_request(ip_address)).await;

        assert_eq!(
            response.status(),
            StatusCode::OK,
            "the bucket of {ip_address} should be untouched by the other address"
        );
    }

    // A silently broken subject resolution (for example, one that hashes every anonymous
    // request into the same bucket, or one that never persists a bucket at all) would also
    // produce two 200s above. Spending the quota of the first address again proves its bucket
    // was really tracked.
    assert_eq!(
        call_service(&service, login_request("10.10.4.88"))
            .await
            .status(),
        StatusCode::TOO_MANY_REQUESTS,
        "the first address had already spent its quota of one request"
    );
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_authenticated_requests_are_limited_per_user(pool: PgPool) {
    let service = rate_limited_service(
        pool,
        rate_limits(
            r#"
default:
  requests: 100
  per_seconds: 60
rules:
  - path_prefix: /api/users/me
    method: GET
    requests: 1
    per_seconds: 60
"#,
        ),
    )
    .await;

    let first_user = login_as(&service, TestUser::Standard).await;
    let second_user = login_as(&service, TestUser::EditArtist).await;

    // Both users share the same ip address, so only a per user bucket separates them.
    let me_request = |token: &str| {
        TestRequest::get()
            .uri("/api/users/me")
            .insert_header(("X-Forwarded-For", "10.10.4.88"))
            .insert_header(auth_header(token))
            .to_request()
    };

    assert_eq!(
        call_service(&service, me_request(&first_user.token))
            .await
            .status(),
        StatusCode::OK
    );
    assert_eq!(
        call_service(&service, me_request(&first_user.token))
            .await
            .status(),
        StatusCode::TOO_MANY_REQUESTS
    );
    assert_eq!(
        call_service(&service, me_request(&second_user.token))
            .await
            .status(),
        StatusCode::OK,
        "the second user has its own bucket even behind the same ip address"
    );
}

#[sqlx::test(fixtures("with_test_users"), migrations = "../storage/migrations")]
async fn test_an_unmapped_method_is_counted_against_the_default_quota(pool: PgPool) {
    let service = rate_limited_service(
        pool,
        rate_limits(
            r#"
default:
  requests: 1
  per_seconds: 60
rules:
  - path_prefix: /api/auth/login
    method: POST
    requests: 100
    per_seconds: 60
"#,
        ),
    )
    .await;

    // HEAD is not a method the rate limiting rules can name, so it cannot match the POST-only
    // rule above; it must still fall on, and be counted against, the default quota rather than
    // pass through unlimited. "/api/auth/login" requires no authentication regardless of
    // method, so this reaches the limiter without a valid session.
    let head_login_request = || {
        TestRequest::default()
            .method(Method::HEAD)
            .uri("/api/auth/login")
            .insert_header(("X-Forwarded-For", "10.10.4.88"))
            .to_request()
    };

    call_service(&service, head_login_request()).await;
    let response = call_service(&service, head_login_request()).await;

    assert_eq!(
        response.status(),
        StatusCode::TOO_MANY_REQUESTS,
        "the second HEAD request should have exhausted the default quota of one"
    );
}
