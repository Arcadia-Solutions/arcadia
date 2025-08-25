use arcadia_cron_jobs::{periodic_tasks::scheduler::run_periodic_tasks, store::Store};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    if env::var("ENV").unwrap_or("".to_string()) != "Docker" {
        dotenvy::from_filename(".env").expect("cannot load env from a file");
    }

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));

    let store = Arc::new(Store::new().await);
    if let Err(e) = run_periodic_tasks(store).await {
        eprintln!("Error running cron tasks: {e:?}");
    }
}
