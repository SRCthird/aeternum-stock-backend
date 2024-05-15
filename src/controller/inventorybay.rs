use diesel::prelude::*;
use rocket::{
    serde::json::Json,
    http::Status,
    response::status,
};

use crate::database::{
    self,
    models::inventorybay::{InventoryBay, CreateInventoryBay, UpdateInventoryBay},
    schema::inventorybay::dsl::*,
};

#[post("/", data = "<_inventorybay>")]
pub fn input(_inventorybay: Json<CreateInventoryBay>) -> Json<InventoryBay> {
    use crate::database::schema::inventorybay;

    let connection = &mut database::establish_connection();
    diesel::insert_into(inventorybay::table)
        .values(_inventorybay.into_inner())
        .execute(connection)
        .expect("Error adding sighting");

    Json(inventorybay::table
        .order(inventorybay::id.desc())
        .first(connection).unwrap()
    )
}

#[get("/")]
pub fn get() -> Json<Vec<InventoryBay>> {
    let connection = &mut database::establish_connection();
    inventorybay.load::<InventoryBay>(connection)
        .map(Json)
        .expect("Error loading birds")
}

#[get("/<_id>")]
pub fn get_one(_id: i32) -> Result<Json<InventoryBay>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = inventorybay
         .filter(id.eq(_id))
         .first::<InventoryBay>(connection);

    match result {
        Ok(found_inventorybay) => Ok(Json(found_inventorybay)),
        Err(_) => Err(status::Custom(Status::NotFound, "InventoryBay not found".to_string())),
    }
}

#[patch("/<_id>", data = "<patch_inventorybay>")]
pub fn update(_id: i32, patch_inventorybay: Json<UpdateInventoryBay>) -> Result<Json<InventoryBay>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    
    let update_result = diesel::update(inventorybay.filter(id.eq(_id)))
        .set(&patch_inventorybay.into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => {
            match inventorybay.filter(id.eq(_id)).first::<InventoryBay>(connection) {
                Ok(updated_inventorybay) => Ok(Json(updated_inventorybay)),
                Err(_) => Err(status::Custom(Status::NotFound, "InventoryBay not found after update".to_string())),
            }
        },
        Err(_) => Err(status::Custom(Status::InternalServerError, "Error updating inventorybay".to_string())),
    }
}

#[delete("/<_id>")]
pub fn delete(_id: i32) -> Result<Json<InventoryBay>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = inventorybay
         .filter(id.eq(_id))
         .first::<InventoryBay>(connection);

    match result {
        Ok(found_inventorybay) => {
            diesel::delete(inventorybay.filter(id.eq(_id)))
                .execute(connection).expect("Error deleting sighting");
            Ok(Json(found_inventorybay))
        },
        Err(_) => Err(status::Custom(Status::NotFound, "InventoryBay not found".to_string())),
    }
}
