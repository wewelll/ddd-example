use std::sync::Arc;

use crate::service::ProductService;
use axum::extract::{Json, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct CreateProduct {
    name: String,
    price: f32,
}

pub async fn list_products(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match state.product_service.list_products().await {
        Ok(products) => (
            StatusCode::OK,
            axum::response::Json(products).into_response(),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::response::Json(json!({
                "error": "Failed to list products"
            }))
            .into_response(),
        ),
    }
}

pub async fn create_product(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProduct>,
) -> impl IntoResponse {
    match state
        .product_service
        .create_product(payload.name, payload.price)
        .await
    {
        Ok(product) => (
            StatusCode::OK,
            axum::response::Json(product).into_response(),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            axum::response::Json(json!({
                "error": "Failed to create product"
            }))
            .into_response(),
        ),
    }
}

pub async fn hello_world(Path(name): Path<String>) -> impl IntoResponse {
    axum::response::Json(json!({ "message": format!("Hello, {}!", name) }))
}

pub struct AppState {
    product_service: ProductService,
}

pub fn create_app(product_service: ProductService) -> Router {
    let shared_state = Arc::new(AppState { product_service });

    Router::new()
        .route("/hello-world/:name", get(hello_world))
        .route("/products", post(create_product))
        .route("/products", get(list_products))
        .with_state(shared_state)
}
