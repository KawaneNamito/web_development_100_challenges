use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match self {
            AppError::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_server_error".to_string(),
                    "サーバー内部でエラーが発生しました".to_string(),
                )
            }
            AppError::Validation(ref msg) => (
                StatusCode::BAD_REQUEST,
                "validation_error".to_string(),
                msg.clone(),
            ),
            AppError::NotFound(ref msg) => (
                StatusCode::NOT_FOUND,
                "not_found".to_string(),
                msg.clone(),
            ),
            AppError::Internal(ref e) => {
                tracing::error!("Internal error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_server_error".to_string(),
                    "サーバー内部でエラーが発生しました".to_string(),
                )
            }
        };

        let body = Json(ErrorResponse {
            error: error_code,
            message,
        });

        (status, body).into_response()
    }
}
