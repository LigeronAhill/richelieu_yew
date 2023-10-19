use axum::extract::{FromRef, Path, State};
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use serde_json::json;
use surrealdb::sql::Id;
use crate::model::product::ProductForCreate;
use crate::storage::ModelController;


#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

async fn create_product(
    State(state): State<AppState>,
    Json(product): Json<ProductForCreate>,
) -> impl IntoResponse {
    println!("->> {:<12} - create_product", "HANDLER");
    let product = state.mc.create_product(product).await;
    match product {
        Ok(product) => Json(product).into_response(),
        Err(_) => {
            ("Failed to create product".to_string()).into_response()
        }
    }
}

async fn list_products(
    State(state): State<AppState>,
) -> impl IntoResponse {
    println!("->> {:<12} - list_products", "HANDLER");
    let products = state.mc.list_products().await;
    match products {
        Ok(products) => Json(products).into_response(),
        Err(_) => {
            ("Failed to list products".to_string()).into_response()
        }
    }
}

async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<Id>,
) -> impl IntoResponse {
    println!("->> {:<12} - delete_product", "HANDLER");
    let product = state.mc.delete_product(id).await;
    match product {
        Ok(product) => Json(product).into_response(),
        Err(_) => {
            ("Failed to delete product".to_string()).into_response()
        }
    }
}

pub fn routes(mc: ModelController) -> Router {
    let app_state = AppState { mc };
    Router::new()
        .route("/products",post(create_product))
        .route("/products", get(list_products))
        .route("/products/:id", delete(delete_product))
        .with_state(app_state)
}