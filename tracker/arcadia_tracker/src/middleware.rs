// use actix_web::{dev::ServiceRequest, error::ErrorUnauthorized, web::Data};
// use actix_web::{Error, FromRequest, HttpRequest};
// use futures::future::{ready, Ready};

// pub struct Passkey(pub String);

// impl FromRequest for Passkey {
//     type Error = Error;
//     type Future = Ready<Result<Self, Self::Error>>;

//     fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
//         let passkey = req.path().into_inner();

//         match passkey {
//             Some(key) => ready(Ok(Passkey(key))),
//             None => ready(Err(actix_web::error::ErrorUnauthorized(
//                 "authentication error: missing passkey",
//             ))),
//         }
//     }
// }

// pub async fn authenticate_user(
//     req: ServiceRequest,
//     passkey: Passkey,
// ) -> std::result::Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
//     // if passkey.0 != arc.env.passkey {
//     //     Err((
//     //         ErrorUnauthorized("authentication error: invalid API key"),
//     //         req,
//     //     ))
//     // } else {
//     Ok(req)
//     // }
// }
