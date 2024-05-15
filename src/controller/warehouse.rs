use diesel::prelude::*;
use rocket::{
    serde::json::Json,
    http::Status,
    response::status,
};

use crate::database::{
    self,
    models::warehouse::{Warehouse, CreateWarehouse, UpdateWarehouse},
    schema::warehouse::dsl,
};

#[post("/", data = "<warehouse>")]
pub fn input(warehouse: Json<CreateWarehouse>) -> Result<Json<Warehouse>, status::Custom<String>> {
    use crate::database::schema::warehouse;

    let connection = &mut database::establish_connection();
    let result = diesel::insert_into(warehouse::table)
        .values(warehouse.into_inner())
        .execute(connection);

    match result {
        Ok(_) => {
            let inserted_warehouse = warehouse::table
                .order(dsl::id.desc())
                .first(connection).unwrap();
            Ok(Json(inserted_warehouse))
        },
        Err(_) => Err(status::Custom(Status::InternalServerError, "Error inserting warehouse".to_string())),
    }
}

#[get("/list")]
pub fn list() -> Result<Json<Vec<String>>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = dsl::warehouse
        .select(dsl::name)
        .load::<String>(connection);

    match result {
        Ok(warehouses) => Ok(Json(warehouses)),
        Err(_) => Err(status::Custom(Status::InternalServerError, "Error loading warehouses".to_string())),
    } 
}

#[get("/?<name>")]
pub fn get(name: Option<String>) -> Result<Json<Vec<Warehouse>>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    let query_result: QueryResult<Vec<Warehouse>> = match name {
        Some(name) => {
            dsl::warehouse.filter(dsl::name.like(format!("{}%", name))).load(connection)
        }
        None => dsl::warehouse.load(connection)
    };

    match query_result {
        Ok(warehouses) => Ok(Json(warehouses)),
        Err(_) => Err(status::Custom(Status::InternalServerError, "Error getting warehouses".to_string()))
    }
}

#[get("/<id>")]
pub fn get_one(id: i32) -> Result<Json<Warehouse>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = dsl::warehouse
         .filter(dsl::id.eq(id))
         .first::<Warehouse>(connection);

    match result {
        Ok(found_warehouse) => Ok(Json(found_warehouse)),
        Err(_) => Err(status::Custom(Status::NotFound, "Warehouse not found".to_string())),
    }
}

#[patch("/<id>", data = "<patch_warehouse>")]
pub fn update(id: i32, patch_warehouse: Json<UpdateWarehouse>) -> Result<Json<Warehouse>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    
    let update_result = diesel::update(dsl::warehouse.filter(dsl::id.eq(id)))
        .set(&patch_warehouse.into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => {
            match dsl::warehouse.filter(dsl::id.eq(id)).first::<Warehouse>(connection) {
                Ok(updated_warehouse) => Ok(Json(updated_warehouse)),
                Err(_) => Err(status::Custom(Status::NotFound, "Warehouse not found after update".to_string())),
            }
        },
        Err(_) => Err(status::Custom(Status::InternalServerError, "Error updating warehouse".to_string())),
    }
}

#[delete("/<id>")]
pub fn delete(id: i32) -> Result<Json<Warehouse>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = dsl::warehouse
         .filter(dsl::id.eq(id))
         .first::<Warehouse>(connection);

    match result {
        Ok(found_warehouse) => {
            diesel::delete(dsl::warehouse.filter(dsl::id.eq(id)))
                .execute(connection).expect("Error deleting sighting");
            Ok(Json(found_warehouse))
        },
        Err(_) => Err(status::Custom(Status::NotFound, "Warehouse not found".to_string())),
    }
}
