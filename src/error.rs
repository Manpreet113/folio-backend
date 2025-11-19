use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

pub enum AppError {
    Internal(String),
    Resend(String),
    ValidationError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {}", msg),
            ),
            AppError::Resend(msg) => (
                StatusCode::BAD_GATEWAY,
                format!("Error sending email: {}", msg),
            ),
            AppError::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                msg,
            ),
        };

        let body = Json(serde_json::json!({ "error": error_message }));
        (status, body).into_response()
    }
}