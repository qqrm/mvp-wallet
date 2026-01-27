use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use wallet_domain::DomainError;
use wallet_app::AppError;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("bad request: {0}")]
    BadRequest(&'static str),

    #[error("unsupported currency")]
    UnsupportedCurrency,

    #[error("unauthorized: {0}")]
    Unauthorized(&'static str),

    #[error("forbidden: {0}")]
    Forbidden(&'static str),

    #[error("not found: {0}")]
    NotFound(&'static str),

    #[error("conflict: {0}")]
    Conflict(&'static str),

    #[error("account closed")]
    AccountClosed,

    #[error("internal error: {0}")]
    Internal(&'static str),

    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
}

impl From<AppError> for ApiError {
    fn from(value: AppError) -> Self {
        match value {
            AppError::BadRequest(m) => ApiError::BadRequest(m),
            AppError::UnsupportedCurrency => ApiError::UnsupportedCurrency,
            AppError::Unauthorized(m) => ApiError::Unauthorized(m),
            AppError::Forbidden(m) => ApiError::Forbidden(m),
            AppError::NotFound(m) => ApiError::NotFound(m),
            AppError::Conflict(m) => ApiError::Conflict(m),
            AppError::AccountClosed => ApiError::AccountClosed,
            AppError::Internal(m) => ApiError::Internal(m),
            AppError::Db(e) => ApiError::Db(e),
        }
    }
}

impl From<DomainError> for ApiError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::BadRequest(m) => ApiError::BadRequest(m),
            DomainError::Internal(m) => ApiError::Internal(m),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg, code) = match self {
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, m.to_string(), None),
            ApiError::UnsupportedCurrency => (
                StatusCode::BAD_REQUEST,
                "unsupported currency".to_string(),
                Some("UNSUPPORTED_CURRENCY".to_string()),
            ),
            ApiError::Unauthorized(m) => (StatusCode::UNAUTHORIZED, m.to_string(), None),
            ApiError::Forbidden(m) => (StatusCode::FORBIDDEN, m.to_string(), None),
            ApiError::NotFound(m) => (StatusCode::NOT_FOUND, m.to_string(), None),
            ApiError::Conflict(m) => (StatusCode::CONFLICT, m.to_string(), None),
            ApiError::AccountClosed => (
                StatusCode::CONFLICT,
                "account is closed".to_string(),
                Some("ACCOUNT_CLOSED".to_string()),
            ),
            ApiError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m.to_string(), None),
            ApiError::Db(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string(), None),
        };

        (
            status,
            Json(ErrorBody {
                error: msg,
                code,
            }),
        )
            .into_response()
    }
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
}
