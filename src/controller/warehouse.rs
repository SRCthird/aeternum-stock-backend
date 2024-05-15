use diesel::prelude::*;
use rocket::{
    serde::json::Json,
    http::Status,
    response::status,
};

use crate::database::{
    self,
    models::warehouse::{Warehouse, CreateWarehouse, UpdateWarehouse},
    schema::warehouse::dsl::*,
};

#[post("/", data = "<_warehouse>")]
pub fn input(_warehouse: Json<CreateWarehouse>) -> Json<Warehouse> {
    use crate::database::schema::warehouse;

    let connection = &mut database::establish_connection();
    diesel::insert_into(warehouse::table)
        .values(_warehouse.into_inner())
        .execute(connection)
        .expect("Error adding sighting");

    Json(warehouse::table
        .order(warehouse::id.desc())
        .first(connection).unwrap()
    )
}

#[get("/")]
pub fn get() -> Json<Vec<Warehouse>> {
    let connection = &mut database::establish_connection();
    warehouse.load::<Warehouse>(connection)
        .map(Json)
        .expect("Error loading birds")
}

#[get("/<_id>")]
pub fn get_one(_id: i32) -> Result<Json<Warehouse>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = warehouse
         .filter(id.eq(_id))
         .first::<Warehouse>(connection);

    match result {
        Ok(found_warehouse) => Ok(Json(found_warehouse)),
        Err(_) => Err(status::Custom(Status::NotFound, "Warehouse not found".to_string())),
    }
}

#[patch("/<_id>", data = "<patch_warehouse>")]
pub fn update(_id: i32, patch_warehouse: Json<UpdateWarehouse>) -> Result<Json<Warehouse>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    
    let update_result = diesel::update(warehouse.filter(id.eq(_id)))
        .set(&patch_warehouse.into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => {
            match warehouse.filter(id.eq(_id)).first::<Warehouse>(connection) {
                Ok(updated_warehouse) => Ok(Json(updated_warehouse)),
                Err(_) => Err(status::Custom(Status::NotFound, "Warehouse not found after update".to_string())),
            }
        },
        Err(_) => Err(status::Custom(Status::InternalServerError, "Error updating warehouse".to_string())),
    }
}

#[delete("/<_id>")]
pub fn delete(_id: i32) -> Result<Json<Warehouse>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = warehouse
         .filter(id.eq(_id))
         .first::<Warehouse>(connection);

    match result {
        Ok(found_warehouse) => {
            diesel::delete(warehouse.filter(id.eq(_id)))
                .execute(connection).expect("Error deleting sighting");
            Ok(Json(found_warehouse))
        },
        Err(_) => Err(status::Custom(Status::NotFound, "Warehouse not found".to_string())),
    }
}
