use actix_web::{HttpResponse, HttpResponseBuilder};
use serde::Serialize;

pub mod error;
pub mod handlers;
pub mod models;

pub trait HttpResponseBuilderExt {
    fn bencode(&mut self, val: impl Serialize) -> HttpResponse;
}

impl HttpResponseBuilderExt for HttpResponseBuilder {
    fn bencode(&mut self, val: impl Serialize) -> HttpResponse {
        match serde_bencode::to_bytes(&val) {
            Ok(data) => self.body(data),
            Err(_) => HttpResponse::BadRequest().body("Failed to bencode"),
        }
    }
}
