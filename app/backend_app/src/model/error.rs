use axum::http::StatusCode;
use usecase::model::error::UsecaseError;

pub struct AppError(pub UsecaseError);

impl Into<StatusCode> for AppError {
    #[track_caller]
    fn into(self) -> StatusCode {
        match self.0 {
            UsecaseError::ValidateError => StatusCode::BAD_REQUEST,
            UsecaseError::Unauthorized => StatusCode::UNAUTHORIZED,
            UsecaseError::Forbidden => StatusCode::FORBIDDEN,
            UsecaseError::NotFound => StatusCode::NOT_FOUND,
            UsecaseError::BadRequest => StatusCode::BAD_REQUEST,
            UsecaseError::InternalServerError { message, file, line, column } => {
                tracing::error!(
                    "Internal Server Error at {}:{}:{} - {}",
                    file,
                    line,
                    column,
                    message
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl From<UsecaseError> for AppError {
    fn from(e: UsecaseError) -> Self {
        AppError(e)
    }
}
