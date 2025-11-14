use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
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
    InternalServerError {
        message: String,
        file: &'static str,
        line: u32,
        column: u32,
    },
}

impl UsecaseError {
    #[track_caller]
    pub fn internal_server_error<E>(err: E) -> Self
    where
        E: Into<anyhow::Error>,
    {
        let e = err.into();
        let loc = std::panic::Location::caller();
        UsecaseError::InternalServerError {
            message: e.to_string(),
            file: loc.file(),
            line: loc.line(),
            column: loc.column(),
        }
    }

    #[track_caller]
    pub fn internal_server_error_msg<S: Into<String>>(msg: S) -> Self {
        let loc = std::panic::Location::caller();
        UsecaseError::InternalServerError {
            message: msg.into(),
            file: loc.file(),
            line: loc.line(),
            column: loc.column(),
        }
    }
}
