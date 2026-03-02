use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use validator::Validate;
use crate::api::dto::{AuthResponseDto, LoginRequest, RegisterRequest};
use crate::api::response::ApiError;
use crate::state::AppState;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>
) -> Result<(StatusCode, Json<AuthResponseDto>), ApiError>{
    payload.validate().map_err(ApiError::from_validation_errors)?;

    let auth_response = state.auth_service.register(payload.email, payload.password, payload.full_name)
        .await
        .map_err(ApiError::from)?;

    Ok((StatusCode::CREATED, Json(AuthResponseDto::from(auth_response))))

}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>
) -> Result<(StatusCode, Json<AuthResponseDto>), ApiError>{
    payload.validate().map_err(ApiError::from_validation_errors)?;

    let auth_response = state.auth_service.login(&payload.email, &payload.password)
        .await
        .map_err(ApiError::from)?;

    Ok((StatusCode::OK, Json(AuthResponseDto::from(auth_response))))
}