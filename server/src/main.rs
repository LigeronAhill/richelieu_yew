pub use self::error::Result;
use std::net::SocketAddr;
use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;
use crate::routes::main_response_mapper;
use crate::storage::ModelController;

mod token;
mod error;
mod routes;
mod model;
mod storage;
mod ctx;
mod log;

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await;

    let routes_api = routes::products::routes(mc.clone())
        .route_layer(middleware::from_fn(routes::mw_auth::mw_require_auth));

    // region: ---Routes
    let routes = Router::new()
        .merge(routes::login())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            routes::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes::nest());
    // endregion: ---Routes

    // region: ---Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->>> Start Server at http://127.0.0.1:8080");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .expect("server failed to start");
    Ok(())
    // endregion: ---Start Server
}