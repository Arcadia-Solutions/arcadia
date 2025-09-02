use crate::Arcadia;
use actix_web::{
    dev::{Payload, ServiceRequest},
    error::ErrorUnauthorized,
    web::Data,
    Error, FromRequest, HttpMessage as _, HttpRequest,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use arcadia_storage::{
    models::user::{Claims, UserClass},
    redis::RedisPoolInterface,
};
use futures_util::future::{err, ok, Ready};
use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey, Validation};

#[derive(Debug, Clone)]
pub struct Authdata {
    pub sub: i64,
    pub class: UserClass,
}

impl FromRequest for Authdata {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        req.extensions()
            .get::<Authdata>()
            .cloned()
            .map(ok)
            .unwrap_or_else(|| err(ErrorUnauthorized("not authorized")))
    }
}

pub async fn authenticate_user<R: RedisPoolInterface + 'static>(
    req: ServiceRequest,
    bearer: Option<BearerAuth>,
) -> std::result::Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    // These routes are explicitly not authenticated.
    if matches!(
        req.path(),
        "/api/auth/login" | "/api/auth/register" | "/api/auth/refresh-token" | "/api/auth/apply"
    ) {
        return Ok(req);
    }

    if let Some(bearer) = bearer {
        validate_bearer_auth::<R>(req, bearer).await
    } else if let Some(api_key) = req.headers().get("api_key") {
        let api_key = api_key.to_str().expect("api_key malformed").to_owned();
        validate_api_key::<R>(req, &api_key).await
    } else {
        Err((
            ErrorUnauthorized("authentication error, missing jwt token or API key"),
            req,
        ))
    }
}

async fn validate_bearer_auth<R: RedisPoolInterface + 'static>(
    req: ServiceRequest,
    bearer: BearerAuth,
) -> std::result::Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let arc = req.app_data::<Data<Arcadia<R>>>().expect("app data set");
    let decoding_key = DecodingKey::from_secret(arc.jwt_secret.as_ref());
    let validation = Validation::default();

    let token_data = match decode::<Claims>(bearer.token(), &decoding_key, &validation) {
        Ok(data) => data,
        Err(err) => {
            return Err((
                match err.kind() {
                    ErrorKind::ExpiredSignature => ErrorUnauthorized("jwt token expired"),
                    _ => ErrorUnauthorized("authentication error"),
                },
                req,
            ));
        }
    };

    let user_id = token_data.claims.sub;

    match arc
        .auth
        .is_invalidated(user_id, token_data.claims.iat)
        .await
    {
        Ok(is_invalidated) if is_invalidated => {
            return Err((ErrorUnauthorized("token for user invalidated"), req))
        }
        Ok(_) => {
            let _ = arc.pool.update_last_seen(user_id).await;
            req.extensions_mut().insert(Authdata {
                sub: user_id,
                class: token_data.claims.class,
            });
        }
        Err(e) => return Err((ErrorUnauthorized(e.to_string()), req)),
    };

    Ok(req)
}

async fn validate_api_key<R: RedisPoolInterface + 'static>(
    req: ServiceRequest,
    api_key: &str,
) -> std::result::Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let arc = req.app_data::<Data<Arcadia<R>>>().expect("app data set");

    let user = match arc.pool.find_user_id_with_api_key(api_key).await {
        Ok(user) => user,
        Err(e) => return Err((actix_web::error::ErrorUnauthorized(e.to_string()), req)),
    };

    req.extensions_mut().insert(Authdata {
        sub: user.id,
        class: user.class,
    });

    Ok(req)
}
