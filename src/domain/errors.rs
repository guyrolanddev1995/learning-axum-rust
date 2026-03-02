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

    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("Password does not match")]
    PasswordDoesNotMatch,

    #[error("Invalid token")]
    InvalidTokenError,

    #[error("Token expired")]
    TokenExpiredError,

    #[error("Unauthorized")]
    UnauthorizedError,

    #[error("Invalid credentials")]
    InvalidCredentialsError,

    #[error("Forbidden: insufficient permissions")]
    InvalidRequestError,
}