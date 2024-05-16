use crate::database::{self, models::inventory::Inventory, schema::inventory::dsl};
use diesel::{prelude::*, result::Error::NotFound};
use rocket::{http::Status, response::status, serde::json::Json};

#[delete("/<id>")]
pub fn delete(id: i32) -> Result<Json<Inventory>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let found_inventory = dsl::inventory
        .filter(dsl::id.eq(id))
        .first::<Inventory>(connection)
        .map_err(|e| match e {
            NotFound => status::Custom(
                Status::NotFound,
                "Inventory not found in system".to_string(),
            ),
            _ => status::Custom(
                Status::InternalServerError,
                "Error loading inventory".to_string(),
            ),
        })?;

    let result = diesel::delete(dsl::inventory.filter(dsl::id.eq(id))).execute(connection);

    match result {
        Ok(_) => Ok(Json(found_inventory)),
        Err(_) => Err(status::Custom(
            Status::NotFound,
            "Inventory not found".to_string(),
        )),
    }
}
