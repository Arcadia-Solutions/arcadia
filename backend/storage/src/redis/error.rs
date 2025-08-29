use arcadia_common::error::Error as CommonError;
use deadpool::managed::PoolError;
pub type Result<T> = std::result::Result<T, RedisError>;

#[derive(Debug, thiserror::Error)]
pub enum RedisError {
    #[error("connection error")]
    ConnectionError(#[source] PoolError<redis::RedisError>),
    #[error("cmd execution error")]
    CmdError(#[source] redis::RedisError),
}

impl From<RedisError> for CommonError {
    fn from(value: RedisError) -> Self {
        CommonError::RedisError(value.to_string())
    }
}
