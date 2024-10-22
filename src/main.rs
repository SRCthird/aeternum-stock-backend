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
            "/inventory/",
            catchers![
                catcher::not_found
            ]
        )
        .attach(middleware::cors::cors().to_cors().unwrap())
        .attach(middleware::apikey::ApiKeyFairing)
        .mount(
            "/inventory/api/error/", 
            routes![
                middleware::apikey::invalid,
                middleware::cors::good
            ]
        )
        .mount(
            "/inventory/api/user/",
            routes![
                user::input,
                user::get,
                user::get_one,
                user::update,
                user::delete
            ],
        )
        .mount(
            "/inventory/api/warehouse/",
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
            "/inventory/api/inventory-bay/",
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
            "/inventory/api/inventory/",
            routes![
                inventory::input,
                inventory::get,
                inventory::get_count,
                inventory::get_one,
                inventory::update,
                inventory::delete
            ],
        )
        .mount(
            "/inventory/api/product/",
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
            "/inventory/api/product-lot/",
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
            "/inventory/api/log/",
            routes![
                log::input, 
                log::get, 
                log::get_one, 
                log::update, 
                log::delete
            ],
        )
}
