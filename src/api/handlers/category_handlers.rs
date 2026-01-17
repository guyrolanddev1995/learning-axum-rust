use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use validator::Validate;
use crate::api::dto::category_dto::{CategoryResponse, CategoryTreeResponse, CreateCategoryDto, UpdateCategoryDto};
use crate::api::response::ApiError;
use crate::state::AppState;

pub async fn category_list(State(state): State<AppState>) -> Result<Json<Vec<CategoryResponse>>, ApiError> {
    let categories = state.category_service.list_all_categories().await?;
    let responses = categories.into_iter().map(CategoryResponse::from).collect();
    Ok(Json(responses))
}

pub async fn list_root_categories(State(state): State<AppState>) -> Result<Json<Vec<CategoryResponse>>, ApiError> {
    let categories = state.category_service.list_root_categories().await?;
    let response = categories.into_iter().map(CategoryResponse::from).collect();
    Ok(Json(response))
}

pub async fn get_category_tree(State(state): State<AppState>) -> Result<Json<Vec<CategoryTreeResponse>>, ApiError> {
    let tree = state.category_service.get_category_tree().await?;
    let response = tree.into_iter().map(CategoryTreeResponse::from).collect();
    Ok(Json(response))
}

pub async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<Json<CategoryResponse>, ApiError> {
    let category = state.category_service.get_category(id).await?;
    Ok(Json(CategoryResponse::from(category)))
}

pub async fn get_category_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>
) -> Result<Json<CategoryResponse>, ApiError> {
    let category = state.category_service.get_category_by_slug(&slug).await?;
    Ok(Json(CategoryResponse::from(category)))
}

pub async fn get_category_children(
    State(state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<Json<Vec<CategoryResponse>>, ApiError> {
    let children = state.category_service.list_children(id).await?;
    let response = children.into_iter().map(CategoryResponse::from).collect();
    Ok(Json(response))
}

pub async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryDto>
) -> Result<(StatusCode, Json<CategoryResponse>), ApiError> {
     payload.validate().unwrap();

    let category = state
        .category_service
        .create_category(payload.name, payload.description, payload.parent_id)
        .await?;

    Ok((StatusCode::CREATED, Json(CategoryResponse::from(category))))
}

pub async fn update_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCategoryDto>
) -> Result<Json<CategoryResponse>, ApiError> {
    payload.validate().unwrap();

    let category = state
        .category_service
        .update_category(id, payload.name, payload.description, payload.parent_id)
        .await?;

    Ok(Json(CategoryResponse::from(category)))
}

pub async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<StatusCode, ApiError> {
    state.category_service.delete_category(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn activate_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<Json<CategoryResponse>, ApiError> {
    let category = state.category_service.activate_category(id).await?;
    Ok(Json(CategoryResponse::from(category)))
}

pub async fn deactivate_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<Json<CategoryResponse>, ApiError> {
    let category = state.category_service.deactivate_category(id).await?;
    Ok(Json(CategoryResponse::from(category)))
}