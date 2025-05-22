use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZKError {
    #[error("Connection error")]
    ConnectionError,

    #[error("Response error")]
    ResponseError,

    #[error("Network error")]
    NetworkError,

    #[error("Unsupported Record Size")]
    UnsupportedRecordSize,
}
