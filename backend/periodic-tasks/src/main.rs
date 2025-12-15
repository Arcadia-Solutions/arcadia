use arcadia_periodic_tasks::{periodic_tasks::scheduler::run_periodic_tasks, store::Store};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    if env::var("ENV").unwrap_or_default() != "Docker" {
        dotenvy::from_filename(".env").expect("cannot load env from a file");
    }

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    log::info!("Starting periodic tasks scheduler...");

    let store = Arc::new(Store::new().await);

    if let Err(e) = run_periodic_tasks(store).await {
        log::error!("Error running periodic tasks: {e:?}");
        std::process::exit(1);
    }
}
