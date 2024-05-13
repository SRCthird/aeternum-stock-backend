use axum::{response::IntoResponse, routing::get, Router, Server};
use diesel::{
    mysql::MysqlConnection,
    r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;
use std::env;

mod models;
mod schema;

type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn get_connection_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    return Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");
}

async fn index(axum::Extension(pool): axum::Extension<DbPool>) -> impl IntoResponse {
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
