use axum::http::StatusCode;
use usecase::model::error::UsecaseError;

pub struct AppError(pub UsecaseError);

impl Into<StatusCode> for AppError {
    fn into(self) -> StatusCode {
        match self.0 {
            UsecaseError::ValidateError => StatusCode::BAD_REQUEST,
            UsecaseError::Unauthorized => StatusCode::UNAUTHORIZED,
            UsecaseError::Forbidden => StatusCode::FORBIDDEN,
            UsecaseError::NotFound => StatusCode::NOT_FOUND,
            UsecaseError::BadRequest => StatusCode::BAD_REQUEST,
            UsecaseError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl From<UsecaseError> for AppError {
    fn from(e: UsecaseError) -> Self {
        AppError(e)
    }
}