use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::http::{Method, Uri};
use axum::routing::{get_service, post};
use serde_json::json;

use tower_http::services::ServeDir;
use uuid::Uuid;
use crate::ctx::Ctx;
use crate::error::Error;
use crate::log::log_request;
use crate::model::api_login;
pub mod products;
pub mod mw_auth;
pub async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RESPONSE_MAPPER");
    let uuid = Uuid::new_v4();
    println!("->> {:<12} - uuid: {}", "RESPONSE_MAPPER", uuid);
    // -- Get the eventual response error

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error
        .map(|se| se.client_status_and_error());

    // -- If client error -> build the new response.
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            println!("    ->> client_error_body: {client_error_body}");

            // Build the new response from the client error
            (*status_code, Json(client_error_body)).into_response()

        });

    let client_error = client_status_error.unzip().1;
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;


    println!("    ->> server log line - {uuid} - Error: {service_error:?}");

    println!();
    error_response.unwrap_or(res)
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
