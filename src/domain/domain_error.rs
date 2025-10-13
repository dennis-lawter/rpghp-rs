#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("Not found")]
    NotFound,
    // #[error("Invalid auth provided")]
    // Unauthorized,
    #[error("Provided auth does not grant permission for requested record")]
    Forbidden,
    #[error("SQL error: {0}")]
    SqlxError(sqlx::Error),
    #[error("Invalid UUID")]
    InvalidUuid(uuid::Error),
}
pub type DomainResult<T> = Result<T, DomainError>;
