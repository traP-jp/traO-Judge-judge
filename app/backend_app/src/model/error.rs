use axum::http::StatusCode;
use usecase::model::error::UsecaseError;

pub struct AppError(pub UsecaseError);

impl From<AppError> for StatusCode {
    #[track_caller]
    fn from(val: AppError) -> Self {
        match val.0 {
            UsecaseError::ValidateError => StatusCode::BAD_REQUEST,
            UsecaseError::Unauthorized => StatusCode::UNAUTHORIZED,
            UsecaseError::Forbidden => StatusCode::FORBIDDEN,
            UsecaseError::NotFound => StatusCode::NOT_FOUND,
            UsecaseError::BadRequest => StatusCode::BAD_REQUEST,
            UsecaseError::InternalServerError {
                message,
                file,
                line,
                column,
            } => {
                tracing::error!(
                    "Internal Server Error at {}:{}:{} - {}",
                    file,
                    line,
                    column,
                    message
                );
                #[cfg(feature = "prod")]
                {
                    // message に 機密情報が含まれてたらまずいのでとりあえず "todo" にしておく
                    let error_message =
                        format!("Location: {}:{}:{}\nmessage: todo", file, line, column);
                    tokio::spawn(async move {
                        traq_log::send_error_message(Some("INTERNAL SERVER ERROR"), &error_message)
                            .await;
                    });
                }
                StatusCode::INTERNAL_SERVER_ERROR
            }
            UsecaseError::ConflictError => StatusCode::CONFLICT,
        }
    }
}

impl From<UsecaseError> for AppError {
    fn from(e: UsecaseError) -> Self {
        AppError(e)
    }
}
