use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use validator::ValidationErrors;
use crate::domain::errors::DomainError;

#[derive(serde::Serialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String
}

pub enum ApiError {
    Standard(StatusCode, String),
    Validation(Vec<ValidationError>)
}


impl ApiError {
    pub fn from_validation_errors(errs: ValidationErrors) -> Self {
        let mut data = Vec::new();

        for (field, errors) in errs.field_errors() {
            for error in errors {
                let message = error.message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| format!("Invalid value for {}", field));

                data.push(ValidationError { field: field.to_string(), message });
            }
        }
        ApiError::Validation(data)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Standard(status_code, message) => {
                (status_code, Json(json!({ "error": message }))).into_response()
            },
            ApiError::Validation(errors) => {
                (StatusCode::UNPROCESSABLE_ENTITY, Json(json!({
                    "error": "Validation error",
                    "data": errors
                }))).into_response()
            }
        }
    }
}

impl From<DomainError> for ApiError {
    fn from(error: DomainError) -> Self {
        match error{
            DomainError::NotFoundError => ApiError::Standard(StatusCode::NOT_FOUND, error.to_string()),
            DomainError::InvalidPriceError => ApiError::Standard(StatusCode::BAD_REQUEST, error.to_string()),
            DomainError::InsufficientStockError => ApiError::Standard(StatusCode::BAD_REQUEST, error.to_string()),
            DomainError::InternalError(message) => ApiError::Standard(StatusCode::INTERNAL_SERVER_ERROR, message),
            DomainError::ValidationError(message) => ApiError::Standard(StatusCode::BAD_REQUEST, message)
        }
    }
}