use actix_web::{middleware, web::Data, App, HttpServer};
use arcadia_tracker::{api_doc::ApiDoc, env::Env, routes::init, Tracker};
use envconfig::Envconfig;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if env::var("ENV").unwrap_or("".to_string()) != "Docker" {
        dotenvy::from_filename(".env").expect("cannot load env from a file");
    }

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let env = Env::init_from_env().unwrap();

    let web_server_port = env::var("WEB_SERVER_PORT").expect("env var WEB_SERVER_PORT must be set");
    let server_url = format!("127.0.0.1:{web_server_port}").to_string();
    println!("Server running at http://{server_url}");

    let arc = Data::new(Tracker::new(env));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(arc.clone())
            .configure(init) // Initialize routes
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/swagger-json/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(server_url)?
    .run();

    server.await
}
