use crate::api::dto::product_dto::{AdjustStockRequest, CreateProductRequest, ProductResponse, ProductWithCategoryResponse, UpdateProductRequest};
use crate::api::response::ApiError;
use crate::state::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use validator::Validate;
use crate::api::extractors::AppJson;

pub async fn list_products(
    State(state): State<AppState>
) -> Result<Json<Vec<ProductResponse>>, ApiError>{
    let products = state.product_service.list_products().await?;
    let responses = products.into_iter().map(ProductResponse::from).collect();
    Ok(Json(responses))
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<Json<ProductResponse>, ApiError>{
    let product = state.product_service.find_product(id).await?;
    Ok(Json(ProductResponse::from(product)))
}

pub async fn get_product_with_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<Json<ProductWithCategoryResponse>, ApiError> {
    let product = state.product_service.find_product(id).await?;
    let category = state.category_service.get_category(product.category_id).await?;

    let response = ProductWithCategoryResponse {
        product: ProductResponse::from(product),
        category_name: category.name,
        category_slug: category.slug
    };

    Ok(Json(response))
}

pub async fn create_product(
    State(app_state): State<AppState>,
    AppJson(payload): AppJson<CreateProductRequest>
) -> Result<(StatusCode, Json<ProductResponse>), ApiError>{
    payload.validate().map_err(ApiError::from_validation_errors)?;

    let product = app_state.product_service.create_product(
        payload.name,
        payload.category_id,
        payload.description,
        payload.price,
        payload.stock,
        payload.sku,
    )
        .await?;

    Ok((StatusCode::CREATED, Json(ProductResponse::from(product))))
}

pub async fn get_products_by_sku(
    State(app_state): State<AppState>,
    Path(sku): Path<String>
) -> Result<Json<ProductResponse>, ApiError>{
    let product = app_state.product_service.find_product_by_sku(&sku).await?;
    Ok(Json(ProductResponse::from(product)))
}

pub async fn get_products_by_category(
    State(app_state): State<AppState>,
    Path(category_id): Path<Uuid>
) -> Result<Json<Vec<ProductResponse>>, ApiError>{
    let products = app_state.product_service.list_products_by_category(category_id).await?;
    let responses = products.into_iter().map(ProductResponse::from).collect();
    Ok(Json(responses))
}

pub async fn get_active_products_by_category(
    State(app_state): State<AppState>,
    Path(category_id): Path<Uuid>
) -> Result<Json<Vec<ProductResponse>>, ApiError>{
    let products = app_state.product_service.list_active_products_by_category(category_id).await?;
    let responses = products.into_iter().map(ProductResponse::from).collect();
    Ok(Json(responses))
}

pub async fn update_product(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    AppJson(payload): AppJson<UpdateProductRequest>
) -> Result<Json<ProductResponse>, ApiError>{
    payload.validate().unwrap();

    let product = app_state.product_service.update_product(
        id,
        payload.category_id,
        payload.name,
        payload.description,
        payload.price,
        payload.stock
    )
        .await?;

    Ok(Json(ProductResponse::from(product)))
}

pub async fn delete_product(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<StatusCode, ApiError>{
    app_state.product_service.delete_product(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn deactivate_product(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<Json<ProductResponse>, ApiError>{
    let product = app_state.product_service.deactivate_product(id).await?;
    Ok(Json(ProductResponse::from(product)))
}

pub async fn activate_product(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<Json<ProductResponse>, ApiError>{
    let product = app_state.product_service.activate_product(id).await?;
    Ok(Json(ProductResponse::from(product)))
}

pub async fn adjust_stock(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    AppJson(payload): AppJson<AdjustStockRequest>
) -> Result<Json<ProductResponse>, ApiError> {
    let product = app_state.product_service.adjust_stock(id, payload.quantity).await?;
    Ok(Json(ProductResponse::from(product)))
}