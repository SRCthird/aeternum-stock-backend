#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

mod database;
mod controller;
use controller::user;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            port: 5000,
            ..Default::default()
        })
        .mount("/", routes![
            user::get,
            user::input,
            user::get_one,
            user::update,
            user::delete
        ])
}

