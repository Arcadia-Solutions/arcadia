use crate::Arcadia;
use actix_web::{dev::ServiceRequest, web, HttpMessage as _};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use arcadia_storage::models::user::Claims;
use jsonwebtoken::{decode, errors::ErrorKind, DecodingKey, Validation};

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

    let banned = arc.pool.is_user_banned(user_id).await;

    if banned {
        return Err((actix_web::error::ErrorUnauthorized("account banned"), req));
    }

    let _ = arc.pool.update_last_seen(user_id).await;

    req.extensions_mut()
        .insert(crate::handlers::UserId(user_id));

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

    let user_id = match arc.pool.find_user_id_with_api_key(api_key).await {
        Ok(id) => id,
        Err(e) => {
            return Err((actix_web::error::ErrorUnauthorized(e.to_string()), req));
        }
    };

    req.extensions_mut()
        .insert(crate::handlers::UserId(user_id));

    Ok(req)
}
