use diesel::prelude::*;
use rocket::{
    serde::json::Json,
    http::Status,
    response::status,
};

use crate::database::{
    self,
    models::productlot::{ProductLot, CreateProductLot, UpdateProductLot},
    schema::productlot::dsl::*,
};

#[post("/", data = "<_productlot>")]
pub fn input(_productlot: Json<CreateProductLot>) -> Json<ProductLot> {
    use crate::database::schema::productlot;

    let connection = &mut database::establish_connection();
    diesel::insert_into(productlot::table)
        .values(_productlot.into_inner())
        .execute(connection)
        .expect("Error adding sighting");

    Json(productlot::table
        .order(productlot::id.desc())
        .first(connection).unwrap()
    )
}

#[get("/")]
pub fn get() -> Json<Vec<ProductLot>> {
    let connection = &mut database::establish_connection();
    productlot.load::<ProductLot>(connection)
        .map(Json)
        .expect("Error loading birds")
}

#[get("/<_id>")]
pub fn get_one(_id: i32) -> Result<Json<ProductLot>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = productlot
         .filter(id.eq(_id))
         .first::<ProductLot>(connection);

    match result {
        Ok(found_productlot) => Ok(Json(found_productlot)),
        Err(_) => Err(status::Custom(Status::NotFound, "ProductLot not found".to_string())),
    }
}

#[patch("/<_id>", data = "<patch_productlot>")]
pub fn update(_id: i32, patch_productlot: Json<UpdateProductLot>) -> Result<Json<ProductLot>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    
    let update_result = diesel::update(productlot.filter(id.eq(_id)))
        .set(&patch_productlot.into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => {
            match productlot.filter(id.eq(_id)).first::<ProductLot>(connection) {
                Ok(updated_productlot) => Ok(Json(updated_productlot)),
                Err(_) => Err(status::Custom(Status::NotFound, "ProductLot not found after update".to_string())),
            }
        },
        Err(_) => Err(status::Custom(Status::InternalServerError, "Error updating productlot".to_string())),
    }
}

#[delete("/<_id>")]
pub fn delete(_id: i32) -> Result<Json<ProductLot>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = productlot
         .filter(id.eq(_id))
         .first::<ProductLot>(connection);

    match result {
        Ok(found_productlot) => {
            diesel::delete(productlot.filter(id.eq(_id)))
                .execute(connection).expect("Error deleting sighting");
            Ok(Json(found_productlot))
        },
        Err(_) => Err(status::Custom(Status::NotFound, "ProductLot not found".to_string())),
    }
}
