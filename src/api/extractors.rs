use axum::extract::{FromRequest, Request};
use axum::Json;
use crate::api::response::ApiError;

pub struct AppJson<T>(pub T);

impl <S, T> FromRequest<S> for AppJson<T>
where
    T: serde::de::DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(Json(value)) => Ok(AppJson(value)),
            Err(rejection) => {
                let message = format!("Invalid JSON: {}", rejection.body_text());
                Err(ApiError::Standard(rejection.status(), message))
            }
        }
    }
}