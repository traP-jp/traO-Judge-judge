use thiserror::Error;

#[derive(Debug, PartialEq, Eq ,Error)]
pub enum UsecaseError {
    #[error("validation error")]
    ValidateError,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
    #[error("Bad Request")]
    BadRequest,
    #[error("Internal Server Error")]
    InternalServerError,
}

impl UsecaseError {
    #[track_caller]
    pub fn internal_server_error<E>(err: E) -> Self
    where
        E: Into<anyhow::Error>,
    {
        tracing::error!("Internal error at {}: {}", std::panic::Location::caller(), err.into());
        UsecaseError::InternalServerError
    }
}