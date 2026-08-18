//! Tests of the `config.yml` parsing. Behaviour built on top of a configuration section
//! belongs to the test file of that feature; this file only covers how the file itself is read.

use arcadia_api::config::{Config, RateLimitsConfig};
use arcadia_storage::models::arcadia_settings::HttpMethod;

#[test]
fn test_rate_limits_configuration_is_parsed() {
    let configuration = r#"
default:
  requests: 300
  per_seconds: 60
rules:
  - path_prefix: /api/auth/login
    method: POST
    requests: 5
    per_seconds: 300
  - path_prefix: /api/search
    requests: 60
    per_seconds: 60
"#;

    let rate_limits: RateLimitsConfig =
        serde_norway::from_str(configuration).expect("the configuration should be parsed");

    assert_eq!(rate_limits.default.requests.get(), 300);
    assert_eq!(rate_limits.default.per_seconds.get(), 60);
    assert_eq!(rate_limits.rules.len(), 2);
    assert_eq!(rate_limits.rules[0].path_prefix, "/api/auth/login");
    assert_eq!(rate_limits.rules[0].method, Some(HttpMethod::Post));
    assert_eq!(rate_limits.rules[0].requests.get(), 5);
    assert_eq!(rate_limits.rules[1].method, None);
}

#[test]
fn test_rate_limits_configuration_rejects_a_zero_quota() {
    let configuration = r#"
default:
  requests: 0
  per_seconds: 60
rules: []
"#;

    let parsed = serde_norway::from_str::<RateLimitsConfig>(configuration);

    assert!(
        parsed.is_err(),
        "a quota of zero request must be rejected when the configuration is loaded"
    );
}

#[test]
fn test_the_allowed_tracker_address_is_optional_and_parsed_as_an_address() {
    let configuration = std::fs::read_to_string("../../config.example.yml")
        .expect("the example configuration should be readable");

    let config: Config =
        serde_norway::from_str(&configuration).expect("the example configuration should be parsed");

    assert!(
        config.tracker.allowed_ip.is_none(),
        "the example configuration leaves the tracker reachable from any address"
    );

    let with_address = configuration.replace(
        "  api_key: change_me",
        "  api_key: change_me\n  allowed_ip: 172.20.0.4",
    );
    let config: Config =
        serde_norway::from_str(&with_address).expect("the configuration should be parsed");

    assert_eq!(
        config.tracker.allowed_ip,
        Some("172.20.0.4".parse().expect("a valid address"))
    );
}
