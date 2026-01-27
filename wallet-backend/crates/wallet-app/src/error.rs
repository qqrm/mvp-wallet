use wallet_domain::DomainError;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
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

impl From<DomainError> for AppError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::BadRequest(m) => AppError::BadRequest(m),
            DomainError::Internal(m) => AppError::Internal(m),
        }
    }
}
