// use actix_http::Request;
// use actix_web::{
//     App, Error,
//     body::MessageBody,
//     dev::{Service, ServiceResponse},
//     http::{
//         StatusCode,
//         header::{AUTHORIZATION, CONTENT_TYPE, HeaderValue, TryIntoHeaderPair},
//     },
//     test, web,
// };
// use arcadia_backend::{Arcadia, OpenSignups, models::user::LoginResponse};
// use reqwest::Url;
// use serde::de::DeserializeOwned;
// use sqlx::PgPool;

// pub async fn create_test_app(
//     pool: PgPool,
//     open_signups: OpenSignups,
//     global_upload_factor: f64,
//     global_download_factor: f64,
// ) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
//     let arc = Arcadia {
//         pool,
//         open_signups,
//         jwt_secret: String::from("verysecurejwtstring"),
//         tracker_name: String::from("Arcadia Test"),
//         frontend_url: Url::parse("http://testurl").unwrap(),
//         tracker_url: Url::parse("http://testurl").unwrap(),
//         tracker_announce_interval: 1800,
//         tracker_announce_interval_grace_period: 1800,
//         allowed_torrent_clients: ["lt0F01".as_bytes().to_vec()].into_iter().collect(),
//         global_upload_factor,
//         global_download_factor,
//         tmdb_api_key: None,
//         smtp_host: None,
//         smtp_port: None,
//         smtp_username: None,
//         smtp_password: None,
//         smtp_from_email: None,
//         smtp_from_name: None,
//     };

//     // TODO: CORS?
//     test::init_service(
//         App::new()
//             .app_data(web::Data::new(arc))
//             .configure(arcadia_backend::routes::init),
//     )
//     .await
// }

// // Requires "with_test_user" fixture.
// pub async fn create_test_app_and_login(
//     pool: PgPool,
//     global_upload_factor: f64,
//     global_download_factor: f64,
// ) -> (
//     impl Service<Request, Response = ServiceResponse, Error = Error>,
//     impl TryIntoHeaderPair,
// ) {
//     let service = create_test_app(
//         pool,
//         OpenSignups::Disabled,
//         global_upload_factor,
//         global_download_factor,
//     )
//     .await;

//     // Login first
//     let req = test::TestRequest::post()
//         .insert_header(("X-Forwarded-For", "10.10.4.88"))
//         .uri("/api/login")
//         .set_json(serde_json::json!({
//             "username": "test_user",
//             "password": "test_password",
//             "remember_me": true,
//         }))
//         .to_request();

//     let user = call_and_read_body_json::<LoginResponse, _>(&service, req).await;

//     assert!(!user.token.is_empty());
//     assert!(!user.refresh_token.is_empty());

//     (service, (AUTHORIZATION, format!("Bearer {}", user.token)))
// }

// pub async fn read_body_bencode<T: DeserializeOwned, B: MessageBody>(
//     resp: ServiceResponse<B>,
// ) -> Result<T, serde_bencode::Error> {
//     let body = test::read_body(resp).await;
//     serde_bencode::from_bytes(&body)
// }

// pub async fn call_and_read_body_json_with_status<T, S>(
//     service: &S,
//     req: Request,
//     status_code: StatusCode,
// ) -> T
// where
//     S: Service<Request, Response = ServiceResponse, Error = Error>,
//     T: DeserializeOwned,
// {
//     let resp = test::call_service(&service, req).await;

//     assert_eq!(
//         resp.status(),
//         status_code,
//         "expected HTTP status {}, got {}",
//         status_code,
//         resp.status()
//     );

//     let content_type = resp.headers().get(CONTENT_TYPE);

//     assert_eq!(
//         content_type,
//         Some(&HeaderValue::from_static("application/json")),
//         "expected Content-Type: application/json, got {content_type:?}"
//     );

//     test::read_body_json::<T, _>(resp).await
// }

// #[inline]
// pub async fn call_and_read_body_json<T, S>(service: &S, req: Request) -> T
// where
//     S: Service<Request, Response = ServiceResponse, Error = Error>,
//     T: DeserializeOwned,
// {
//     call_and_read_body_json_with_status(service, req, StatusCode::OK).await
// }
