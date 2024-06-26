#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

mod controller;
mod database;
mod middleware;
mod catcher;
use controller::{inventory, inventorybay, log, product, productlot, user, warehouse};


#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .register(
            "/",
            catchers![
                catcher::not_found
            ]
        )
        .attach(middleware::cors::cors().to_cors().unwrap())
        .attach(middleware::apikey::ApiKeyFairing)
        .mount(
            "/api/error/", 
            routes![
                middleware::apikey::invalid,
                middleware::cors::good
            ]
        )
        .mount(
            "/api/user/",
            routes![
                user::input,
                user::get,
                user::get_one,
                user::update,
                user::delete
            ],
        )
        .mount(
            "/api/warehouse/",
            routes![
                warehouse::input,
                warehouse::list,
                warehouse::get,
                warehouse::get_one,
                warehouse::update,
                warehouse::delete
            ],
        )
        .mount(
            "/api/inventory-bay/",
            routes![
                inventorybay::input,
                inventorybay::list,
                inventorybay::get,
                inventorybay::get_one,
                inventorybay::update,
                inventorybay::delete
            ],
        )
        .mount(
            "/api/inventory/",
            routes![
                inventory::input,
                inventory::get,
                inventory::get_one,
                inventory::update,
                inventory::delete
            ],
        )
        .mount(
            "/api/product/",
            routes![
                product::input,
                product::list,
                product::get,
                product::get_one,
                product::update,
                product::delete
            ],
        )
        .mount(
            "/api/product-lot/",
            routes![
                productlot::input,
                productlot::get,
                productlot::list,
                productlot::get_one,
                productlot::update,
                productlot::delete
            ],
        )
        .mount(
            "/api/log/",
            routes![
                log::input, 
                log::get, 
                log::get_one, 
                log::update, 
                log::delete
            ],
        )
}
