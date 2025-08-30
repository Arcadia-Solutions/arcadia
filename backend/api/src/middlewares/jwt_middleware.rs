use crate::Arcadia;
use actix_web::{
    dev::{Payload, ServiceRequest},
    error::ErrorUnauthorized,
    web, Error, FromRequest, HttpMessage as _, HttpRequest,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use arcadia_storage::models::user::Claims;
use futures_util::future::{err, ok, Ready};
use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey, Validation};

#[derive(Debug, Clone)]
pub struct Authdata {
    pub sub: i64,
    pub class: String,
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

pub async fn authenticate_user(
    req: ServiceRequest,
    bearer: Option<BearerAuth>,
) -> std::result::Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    // These routes are explicitly not authenticated.
    if matches!(
        req.path(),
        "/api/auth/login"
            | "/api/auth/register"
            | "/api/auth/refresh-token"
            | "/api/user-applications"
    ) {
        return Ok(req);
    }

    if let Some(bearer) = bearer {
        validate_bearer_auth(req, bearer).await
    } else if let Some(api_key) = req.headers().get("api_key") {
        let api_key = api_key.to_str().expect("api_key malformed").to_owned();
        validate_api_key(req, &api_key).await
    } else {
        Err((
            actix_web::error::ErrorUnauthorized(
                "authentication error, missing jwt token or API key",
            ),
            req,
        ))
    }
}

async fn validate_bearer_auth(
    req: ServiceRequest,
    bearer: BearerAuth,
) -> std::result::Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let Some(arc) = req.app_data::<web::Data<Arcadia>>() else {
        return Err((
            actix_web::error::ErrorUnauthorized("authentication error"),
            req,
        ));
    };

    let decoding_key = DecodingKey::from_secret(arc.jwt_secret.as_ref());

    let validation = Validation::default();

    let token_data = match decode::<Claims>(bearer.token(), &decoding_key, &validation) {
        Ok(data) => data,
        Err(err) => {
            return Err((
                match err.kind() {
                    ErrorKind::ExpiredSignature => {
                        actix_web::error::ErrorUnauthorized("jwt token expired")
                    }
                    _ => actix_web::error::ErrorUnauthorized("authentication error"),
                },
                req,
            ));
        }
    };

    let user_id = token_data.claims.sub;
    let Ok(banned) = arc.pool.is_user_banned(user_id).await else {
        return Err((
            actix_web::error::ErrorUnauthorized("account does not exist"),
            req,
        ));
    };

    if banned {
        return Err((actix_web::error::ErrorUnauthorized("account banned"), req));
    }

    let _ = arc.pool.update_last_seen(user_id).await;
    req.extensions_mut().insert(Authdata {
        sub: user_id,
        class: token_data.claims.class,
    });

    Ok(req)
}

async fn validate_api_key(
    req: ServiceRequest,
    api_key: &str,
) -> std::result::Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let Some(arc) = req.app_data::<web::Data<Arcadia>>() else {
        return Err((
            actix_web::error::ErrorUnauthorized("authentication error"),
            req,
        ));
    };

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
