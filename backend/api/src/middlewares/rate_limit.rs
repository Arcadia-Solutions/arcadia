use crate::{
    config::RateLimitsConfig,
    middlewares::{
        auth_middleware::{routed_path, Authdata},
        client_ip, http_method,
    },
    Arcadia,
};
use actix_web::{
    body::{BoxBody, MessageBody},
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web::Data,
    HttpMessage as _,
};
use arcadia_common::error::Error;
use arcadia_storage::{models::arcadia_settings::HttpMethod, redis::RedisPoolInterface};
use governor::{clock::Clock, DefaultKeyedRateLimiter, Quota, RateLimiter};
use std::{
    net::{IpAddr, Ipv6Addr},
    num::NonZeroU32,
    time::Duration,
};

/// What a rate limiting bucket is keyed on: the user when the request is authenticated, the
/// address of the client otherwise.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RateLimitSubject {
    User(i32),
    Ip(IpAddr),
}

/// Outcome of checking one request against the policy.
#[derive(Debug)]
pub enum RateLimitDecision {
    Allowed,
    Rejected {
        /// The `path_prefix` of the matched rule, or "default".
        rule: String,
        retry_after_seconds: u64,
    },
}

/// The name reported for requests that no rule matched.
const DEFAULT_RULE_NAME: &str = "default";

/// How often the limiters are swept. `retain_recent` only drops keys whose allowance is fully
/// replenished, so this is a memory against cpu trade rather than a correctness knob.
pub const SWEEP_INTERVAL: Duration = Duration::from_secs(60);

struct CompiledRule {
    path_prefix: String,
    method: Option<HttpMethod>,
    limiter: DefaultKeyedRateLimiter<RateLimitSubject>,
}

pub struct RateLimitPolicy {
    rules: Vec<CompiledRule>,
    default_limiter: DefaultKeyedRateLimiter<RateLimitSubject>,
}

/// Builds a limiter allowing `requests` immediately, then replenishing the allowance smoothly
/// at `requests` per `per_seconds`.
fn keyed_limiter(
    requests: NonZeroU32,
    per_seconds: NonZeroU32,
) -> DefaultKeyedRateLimiter<RateLimitSubject> {
    let replenish_one_every = Duration::from_secs(u64::from(per_seconds.get())) / requests.get();
    let quota = Quota::with_period(replenish_one_every)
        .expect("a rate limit allows at most one billion requests per configured second")
        .allow_burst(requests);

    RateLimiter::keyed(quota)
}

impl RateLimitPolicy {
    pub fn new(configuration: &RateLimitsConfig) -> Self {
        let rules = configuration
            .rules
            .iter()
            .map(|rule| CompiledRule {
                path_prefix: rule.path_prefix.clone(),
                method: rule.method.clone(),
                limiter: keyed_limiter(rule.requests, rule.per_seconds),
            })
            .collect();

        Self {
            rules,
            default_limiter: keyed_limiter(
                configuration.default.requests,
                configuration.default.per_seconds,
            ),
        }
    }

    /// The `path_prefix` of every configured rule that no endpoint of the API starts with. Such
    /// a rule can never match a request, so it is almost always a typo or a route that was
    /// renamed, and the operator believes an endpoint is protected while it is not.
    pub fn rules_matching_no_endpoint(&self, endpoint_paths: &[String]) -> Vec<&str> {
        self.rules
            .iter()
            .filter(|rule| {
                !endpoint_paths
                    .iter()
                    .any(|endpoint_path| endpoint_path.starts_with(&rule.path_prefix))
            })
            .map(|rule| rule.path_prefix.as_str())
            .collect()
    }

    /// The first rule whose prefix and method match, or the default limiter. `method` is `None`
    /// for a request whose method the rules cannot name (HEAD, OPTIONS, ...), which therefore
    /// only matches a rule applying to every method; the default limiter names no method of its
    /// own, so it stays reachable either way.
    fn limiter_for(
        &self,
        path: &str,
        method: Option<&HttpMethod>,
    ) -> (&str, &DefaultKeyedRateLimiter<RateLimitSubject>) {
        for rule in &self.rules {
            let method_matches = rule.method.is_none() || rule.method.as_ref() == method;

            if method_matches && path.starts_with(&rule.path_prefix) {
                return (rule.path_prefix.as_str(), &rule.limiter);
            }
        }

        (DEFAULT_RULE_NAME, &self.default_limiter)
    }

    pub fn check(
        &self,
        subject: &RateLimitSubject,
        path: &str,
        method: Option<&HttpMethod>,
    ) -> RateLimitDecision {
        let (rule, limiter) = self.limiter_for(path, method);

        match limiter.check_key(subject) {
            Ok(()) => RateLimitDecision::Allowed,
            Err(not_until) => {
                let wait = not_until.wait_time_from(limiter.clock().now());
                let retry_after_seconds =
                    (wait.as_secs() + u64::from(wait.subsec_nanos() > 0)).max(1);

                RateLimitDecision::Rejected {
                    rule: rule.to_owned(),
                    retry_after_seconds,
                }
            }
        }
    }

    /// Drops the keys whose allowance is fully replenished. `governor` keeps one entry per key
    /// forever otherwise, so this is what bounds the memory of the limiters.
    pub fn retain_recent(&self) {
        for rule in &self.rules {
            rule.limiter.retain_recent();
        }
        self.default_limiter.retain_recent();
    }
}

/// Requests below this prefix come from the tracker, which is never rate limited. The
/// authentication middleware has already checked that they carry its api key and come from the
/// allowed address.
const TRACKER_PATH_PREFIX: &str = "/api/tracker";

/// Keys an IPv6 client on its /64 rather than its full address, leaving IPv4 unchanged: a
/// residential or mobile client routed a whole block would otherwise mint an unbounded number
/// of buckets by rotating the address inside it. Canonicalised first so an IPv4-mapped address
/// is not masked as though it were native IPv6.
pub fn mask_ipv6_client_address(address: IpAddr) -> IpAddr {
    let address = address.to_canonical();
    match address {
        IpAddr::V4(_) => address,
        IpAddr::V6(address) => {
            let mut octets = address.octets();
            octets[8..].fill(0);
            IpAddr::V6(Ipv6Addr::from(octets))
        }
    }
}

/// Counts one request against the policy, answering 429 when the quota of its subject is
/// exhausted.
///
/// Wrapped as the innermost middleware of the `/api` scope so it runs after the authentication
/// and can key on the user id. Accepted consequences: every request pays the authentication
/// work before being throttled, and a request whose api key is rejected never reaches here, so
/// guessing an api key is not limited.
pub async fn rate_limit_middleware<R: RedisPoolInterface + 'static>(
    req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<BoxBody>, actix_web::Error> {
    let Some(arc) = req.app_data::<Data<Arcadia<R>>>().cloned() else {
        return Ok(next.call(req).await?.map_into_boxed_body());
    };
    let Some(policy) = arc.rate_limit_policy.clone() else {
        return Ok(next.call(req).await?.map_into_boxed_body());
    };

    if routed_path(&req).starts_with(TRACKER_PATH_PREFIX) {
        return Ok(next.call(req).await?.map_into_boxed_body());
    }

    let ip = client_ip(&req, arc.api.reverse_proxy_client_ip_header_name.as_deref());
    let user_id = req.extensions().get::<Authdata>().map(|data| data.sub);

    let subject = match (user_id, ip) {
        (Some(user_id), _) => RateLimitSubject::User(user_id),
        (None, Some(ip)) => RateLimitSubject::Ip(mask_ipv6_client_address(ip)),
        (None, None) => return Ok(next.call(req).await?.map_into_boxed_body()),
    };

    // The method is only used to match a rule; a method the rules cannot name (HEAD, OPTIONS,
    // ...) still gets checked, it just cannot match a rule that names one, per
    // `RateLimitPolicy::limiter_for`. Skipping the limiter here would exempt such a method from
    // every quota, including the default one.
    let method = http_method(req.method());

    // In its own statement so the borrow `routed_path` takes on `req` ends before the arms
    // below move it.
    let decision = policy.check(&subject, routed_path(&req), method.as_ref());

    match decision {
        RateLimitDecision::Allowed => Ok(next.call(req).await?.map_into_boxed_body()),
        RateLimitDecision::Rejected {
            rule,
            retry_after_seconds,
        } => {
            tracing::warn!(
                rate_limit.user_id = ?user_id,
                rate_limit.ip = ?ip,
                rate_limit.method = %req.method(),
                rate_limit.path = %routed_path(&req),
                rate_limit.rule = %rule,
                "rate limit exceeded"
            );

            // Built as a response rather than propagated as a service error: `from_fn`
            // middlewares do not get their `Err` turned into a response by the framework the
            // way handlers and `HttpAuthentication` do, so the error would otherwise escape the
            // whole middleware chain unconverted.
            Ok(req
                .error_response(Error::RateLimitExceeded {
                    retry_after_seconds,
                })
                .map_into_boxed_body())
        }
    }
}
