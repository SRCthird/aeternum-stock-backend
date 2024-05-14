pub mod models;
pub mod schema;
pub mod error;

use diesel::{
    mysql::MysqlConnection,
    r2d2::{ConnectionManager, Pool},
};
use dotenvy::dotenv;
use std::env;

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn get_connection_pool() -> DbPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    return Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");
}
