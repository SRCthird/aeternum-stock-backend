use crate::database::{self, models::inventory::Inventory, schema::inventory::dsl};
use diesel::prelude::*;

use rocket::{http::Status, response::status, serde::json::Json};
#[get("/")]
pub fn get() -> Result<Json<Vec<Inventory>>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = dsl::inventory.load(connection);

    match result {
        Ok(inventorys) => Ok(Json(inventorys)),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error getting inventorys".to_string(),
        )),
    }
}
