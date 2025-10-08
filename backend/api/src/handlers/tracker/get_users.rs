use crate::Arcadia;
use actix_web::{web::Data, HttpResponse};
use arcadia_common::error::Result;
use arcadia_storage::redis::RedisPoolInterface;
use serde::Serialize;

#[inline]
fn binary_response<T: Serialize>(value: &T) -> Result<HttpResponse> {
    let bytes = serde_bencode::to_bytes(value).expect("error encoding to binary");
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(bytes))
}

pub async fn exec<R: RedisPoolInterface + 'static>(arc: Data<Arcadia<R>>) -> Result<HttpResponse> {
    let users = arc.pool.find_users().await?;
    binary_response(&users)
}
