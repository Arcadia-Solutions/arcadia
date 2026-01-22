use actix_cors::Cors;
use actix_web::{middleware, web::Data, App, HttpServer};
use arcadia_api::routes::init;
use arcadia_api::{api_doc::ApiDoc, env::Env, Arcadia};
use arcadia_periodic_tasks::periodic_tasks::scheduler::run_periodic_tasks;
use arcadia_storage::connection_pool::ConnectionPool;
use arcadia_storage::redis::RedisPool;
use envconfig::Envconfig;
use std::{env, sync::Arc};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if env::var("ENV").unwrap_or("".to_string()) != "Docker" {
        dotenvy::from_filename(".env").expect("cannot load env from a file");
    }

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let env = Env::init_from_env().unwrap();

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

    let tracker_config = arcadia_storage::connection_pool::TrackerConfig {
        url_internal: env.tracker.url_internal.clone(),
        api_key: env.tracker.api_key.clone(),
    };

    // Initialize and start periodic tasks before starting the web server
    // This ensures that if periodic tasks fail to initialize (e.g., missing env var),
    // the entire application fails to start
    let store = Arc::new(arcadia_periodic_tasks::store::Store::new(tracker_config.clone()).await);
    let _scheduler = run_periodic_tasks(store)
        .await
        .expect("Failed to initialize periodic tasks");

    let pool = Arc::new(
        ConnectionPool::try_new(&env.database_url, tracker_config)
            .await
            .expect("db connection"),
    );
    let redis_pool = Arc::new(RedisPool::new(
        &env.redis.host,
        &env.redis.password,
        env.redis.port,
    ));

    // Load settings from database on startup
    let settings = pool
        .get_arcadia_settings()
        .await
        .expect("failed to load arcadia settings from database");

    let arc = Data::new(Arcadia::new(
        Arc::clone(&pool),
        Arc::clone(&redis_pool),
        env,
        settings,
    ));
    let server = HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(arc.clone())
            .configure(init::<RedisPool>) // Initialize routes
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/swagger-json/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(server_url)?
    .run();

    server.await
}
