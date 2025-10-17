use crate::{handlers::tracker::binary_response, Arcadia};
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::redis::RedisPoolInterface;

pub async fn exec<R: RedisPoolInterface + 'static>(arc: Data<Arcadia<R>>) -> Result<HttpResponse> {
    let torrents = arc.pool.find_torrents().await?;
    binary_response(&torrents)
}
