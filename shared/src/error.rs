use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Invalid infohash.")]
    InfoHash,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    DatabseError(String),
    #[error("Decoding error: {0}")]
    DecodingError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl actix_web::ResponseError for Error {
    #[inline]
    fn status_code(&self) -> actix_web::http::StatusCode {
        use actix_web::http::StatusCode;

        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        log::error!("The request generated this error: {self}");
        actix_web::HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
