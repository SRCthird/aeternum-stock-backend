pub mod schema;
pub mod models;
use std::env;
use std::path::Path;
use diesel::prelude::*;
use dotenvy::dotenv;

pub fn establish_connection() -> MysqlConnection {
    let database_url = database_url();

    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn database_url() -> String {
    if let Ok(exe_path) = env::current_exe() {
        let dir = exe_path.parent().unwrap_or_else(|| Path::new(""));
        let dotenv_path = dir.join(".env");
        if dotenvy::from_filename(dotenv_path.to_str().unwrap()).is_err() {
            dotenv().ok();
        }
    } else {
        dotenv().ok();
    }

    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
