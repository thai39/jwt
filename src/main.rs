use axum::{Extension, Router};
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
        .merge(routes::auth_route::auth_routes())
        .merge(routes::user_route::user_routes())
        .layer(Extension(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
