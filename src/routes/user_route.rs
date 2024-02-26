use axum::{http::Method, routing::{delete, put, get, post}, Router};
use tower_http::cors::{CorsLayer, Any};

use crate::handlers::user_handler::{del, get_all_users, update};

pub fn user_routes() -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any);
    let router = Router::new()
        .route("/api/user/update/:uuid", put(update))
        .route("/api/user/delete/:uuid", delete(del))
        .route("/api/user/list", get(get_all_users))
        .layer(cors);
    router
}
