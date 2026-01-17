use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Resource not found")]
    NotFoundError,

    #[error("Invalid price: must be greater than 0")]
    InvalidPriceError,

    #[error("Insufficient stock")]
    InsufficientStockError,

    #[error("Internal error: {0}")]
    InternalError(String),
    
    #[error("{0}")]
    ValidationError(String),
}