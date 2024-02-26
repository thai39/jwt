use axum::{middleware, Extension, Router};
use sea_orm::Database;

mod handlers;
mod models;
mod routes;
mod utils;


#[tokio::main]
async fn main() {
    server().await;
}
async fn server() {
    let conn = (*utils::connectDb::DATABASE_URL).clone();
    let db = Database::connect(conn).await.expect("Failed to connect to db");

    let app = Router::new()
    .merge(routes::user_route::user_routes())
    .route_layer(middleware::from_fn(utils::guards::guard))
    .merge(routes::auth_route::auth_routes())
        .layer(Extension(db));

        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
