use axum::{http::Method, routing::post, Router};
use tower_http::cors::{CorsLayer, Any};
use crate::handlers::auth_handler;

pub fn auth_routes() -> Router {
    
    let cors = CorsLayer::new()
    .allow_methods([Method::POST])
    .allow_origin(Any);

    let router = Router::new()
        .route("/api/register", post(auth_handler::register))
        .route("/api/login", post(auth_handler::login))
        .layer(cors);
    router
}
