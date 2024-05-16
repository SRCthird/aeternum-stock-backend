use crate::database::{self, models::inventory::Inventory, schema::inventory::dsl};
use diesel::prelude::*;
use rocket::{http::Status, response::status, serde::json::Json};

#[get("/<id>")]
pub fn get_one(id: i32) -> Result<Json<Inventory>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = dsl::inventory
        .filter(dsl::id.eq(id))
        .first::<Inventory>(connection);

    match result {
        Ok(found_inventory) => Ok(Json(found_inventory)),
        Err(_) => Err(status::Custom(
            Status::NotFound,
            "Inventory not found".to_string(),
        )),
    }
}
