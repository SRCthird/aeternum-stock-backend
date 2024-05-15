use diesel::prelude::*;
use rocket::{
    serde::json::Json,
    http::Status,
    response::status,
};

use crate::database::{
    self,
    models::inventory::{Inventory, CreateInventory, UpdateInventory},
    schema::inventory::dsl::*,
};

#[post("/", data = "<_inventory>")]
pub fn input(_inventory: Json<CreateInventory>) -> Json<Inventory> {
    use crate::database::schema::inventory;

    let connection = &mut database::establish_connection();
    diesel::insert_into(inventory::table)
        .values(_inventory.into_inner())
        .execute(connection)
        .expect("Error adding sighting");

    Json(inventory::table
        .order(inventory::id.desc())
        .first(connection).unwrap()
    )
}

#[get("/")]
pub fn get() -> Json<Vec<Inventory>> {
    let connection = &mut database::establish_connection();
    inventory.load::<Inventory>(connection)
        .map(Json)
        .expect("Error loading birds")
}

#[get("/<_id>")]
pub fn get_one(_id: i32) -> Result<Json<Inventory>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = inventory
         .filter(id.eq(_id))
         .first::<Inventory>(connection);

    match result {
        Ok(found_inventory) => Ok(Json(found_inventory)),
        Err(_) => Err(status::Custom(Status::NotFound, "Inventory not found".to_string())),
    }
}

#[patch("/<_id>", data = "<patch_inventory>")]
pub fn update(_id: i32, patch_inventory: Json<UpdateInventory>) -> Result<Json<Inventory>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    
    let update_result = diesel::update(inventory.filter(id.eq(_id)))
        .set(&patch_inventory.into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => {
            match inventory.filter(id.eq(_id)).first::<Inventory>(connection) {
                Ok(updated_inventory) => Ok(Json(updated_inventory)),
                Err(_) => Err(status::Custom(Status::NotFound, "Inventory not found after update".to_string())),
            }
        },
        Err(_) => Err(status::Custom(Status::InternalServerError, "Error updating inventory".to_string())),
    }
}

#[delete("/<_id>")]
pub fn delete(_id: i32) -> Result<Json<Inventory>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = inventory
         .filter(id.eq(_id))
         .first::<Inventory>(connection);

    match result {
        Ok(found_inventory) => {
            diesel::delete(inventory.filter(id.eq(_id)))
                .execute(connection).expect("Error deleting sighting");
            Ok(Json(found_inventory))
        },
        Err(_) => Err(status::Custom(Status::NotFound, "Inventory not found".to_string())),
    }
}
