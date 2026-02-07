use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::http::{HeaderMap, HeaderName, Method, StatusCode, Uri, header};
use axum::response::{Html, IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug)]
pub struct Payload {
    name: String,
    age: u32,
}

async fn create_things() -> Result<Payload, StatusCode> {
    Ok(Payload {
        name: "John".to_string(),
        age: 30,
    })
}

pub async fn app_handler(
    uri: Uri,
    method: Method,
    headers: HeaderMap,
    body: Json<Payload>
) -> Result<impl IntoResponse, StatusCode> {
    let path = uri.path();
    let data = create_things().await?;

    Ok((
        StatusCode::OK,
        body
    ))
}
