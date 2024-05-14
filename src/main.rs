pub mod database;
pub mod inventory;

use axum::{response::IntoResponse, routing::get, Router, Server};
use database::{
    DbPool,
    get_connection_pool
};

async fn index(axum::Extension(pool): axum::Extension<DbPool>) -> impl IntoResponse {
    let _conn = pool.get().expect("could not get db connection from pool");
    return "Hello, World!".to_string();
}

#[tokio::main]
async fn main() {
    let pool = get_connection_pool();

    let app = Router::new()
        .route("/", get(index))
        .layer(axum::Extension(pool));

    println!("Running on http://localhost:3000");
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
