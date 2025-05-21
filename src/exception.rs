use thiserror::Error;

#[derive(Debug, Error)]
pub enum ZKError {
    #[error("Connection error")]
    Connection,

    #[error("Response error")]
    Response,

    #[error("Network error")]
    Network,
}
