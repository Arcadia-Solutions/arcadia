mod handlers;
mod models;
mod periodic_tasks;
mod repositories;
mod routes;
mod services;
mod tracker;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web::Data};
use envconfig::Envconfig;
use periodic_tasks::scheduler::run_periodic_tasks;
use routes::init;
use sqlx::postgres::PgPoolOptions;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use arcadia_backend::{Arcadia, Error, Result, api_doc::ApiDoc, env::Env};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if env::var("ENV").unwrap() == "development" {
        dotenvy::from_filename(".env").expect("cannot load env from a file");
    }

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let env = Env::init_from_env().unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env.database_url)
        .await
        .expect("Error building a connection pool");

    let server_url = format!("{}:{}", env.actix.host, env.actix.port);
    println!("Server running at http://{server_url}");

    if env.tmdb_api_key.is_none() {
        println!("TMDB_API_KEY env var is not set. TMDB data fetching won't be available")
    }

    // Log email configuration status
    if env.smtp.host.is_some()
        && env.smtp.port.is_some()
        && env.smtp.username.is_some()
        && env.smtp.password.is_some()
        && env.smtp.from_email.is_some()
        && env.smtp.from_name.is_some()
    {
        println!("Email service configured and enabled");
    } else {
        println!("Email service not configured - emails will be skipped");
    }

    let arc = Data::new(Arcadia::new(pool, env));
    let arc_periodic_tasks = arc.clone();

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(arc.clone())
            .configure(init) // Initialize routes
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/swagger-json/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(server_url)?
    .run();

    tokio::spawn(async {
        if let Err(e) = run_periodic_tasks(arc_periodic_tasks).await {
            eprintln!("Error running cron tasks: {e:?}");
        }
    });

    server.await
}
