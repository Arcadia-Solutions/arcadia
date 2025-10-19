use crate::{handlers::tracker::binary_response, Arcadia};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::Result;
use arcadia_shared::tracker::models::env::Env;
use arcadia_storage::redis::RedisPoolInterface;

pub async fn exec<R: RedisPoolInterface + 'static>(arc: Data<Arcadia<R>>) -> Result<HttpResponse> {
    let env = Env {
        global_download_factor: arc.env.global_download_factor,
        global_upload_factor: arc.env.global_upload_factor,
    };

    binary_response(&env)
}
