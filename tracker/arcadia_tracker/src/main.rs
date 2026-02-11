use actix_web::{web::Data, App, HttpServer};
use arcadia_tracker::{api_doc::ApiDoc, env::Env, routes::init, scheduler, Tracker};
use envconfig::Envconfig;
use std::env;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if env::var("ENV").unwrap_or("".to_string()) != "Docker" {
        dotenvy::from_filename(".env").expect("cannot load env from a file");
    }

    arcadia_shared::telemetry::init_telemetry();

    let env = Env::init_from_env().unwrap();

    let web_server_port = env::var("WEB_SERVER_PORT").expect("env var WEB_SERVER_PORT must be set");
    let web_server_host = env::var("WEB_SERVER_HOST").expect("env var WEB_SERVER_HOST must be set");
    let server_url = format!("{}:{}", web_server_host, web_server_port);
    println!("Server running at http://{server_url}");

    let arc = Data::new(Tracker::new(env).await);

    if let Some(service_name) = &arc.otel_service_name {
        arcadia_tracker::metrics::register(&arc, service_name);
    } else {
        log::info!("OTEL_SERVICE_NAME is not set, skipping metrics registration");
    }

    // Starts scheduler to automate flushing updates
    // to database and inactive peer removal.
    let scheduler_handle = tokio::spawn({
        let arc = arc.clone();

        async move {
            scheduler::handle(&arc).await;
        }
    });

    let arc2 = arc.clone();
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(arc2.clone())
            .configure(init) // Initialize routes
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/swagger-json/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(server_url)?
    .run();

    server.await?;

    // Flush all remaining updates before shutting down.
    let max_flushes = 1000;
    let mut flushes = 0;

    // stop the scheduler to avoid race conditions
    scheduler_handle.abort();

    while flushes < max_flushes
        && (!arc.peer_updates.lock().is_empty()
            || !arc.torrent_updates.lock().is_empty()
            || !arc.user_updates.lock().is_empty())
    {
        scheduler::flush(&arc).await;
        flushes += 1;
    }

    if flushes == max_flushes {
        log::error!("Graceful shutdown failed");
    } else {
        log::info!("Graceful shutdown succeeded");
    }

    Ok(())
}
