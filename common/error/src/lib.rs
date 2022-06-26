use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    InvalidArgument(String),
    #[error("{0}")]
    Internal(String),
}
