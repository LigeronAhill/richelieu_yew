use axum::response::Response;
use axum::Router;
use axum::routing::{get_service, post};
use tower_http::services::ServeDir;
use crate::model::api_login;
pub mod products;
pub mod mw_auth;
pub async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - {res:#?} main_response_mapper", "RESPONSE_MAPPER");
    println!();
    res
}

pub fn nest() -> Router {
    Router::new()
        .nest_service(
            "/",
            get_service(ServeDir::new("./server/"))
                .handle_error(|error| async move {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                )
            }),
        )
}

pub fn login() -> Router {
    Router::new().route("/api/login", post(api_login))
}
