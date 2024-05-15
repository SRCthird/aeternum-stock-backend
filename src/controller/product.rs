use diesel::prelude::*;
use rocket::{
    serde::json::Json,
    http::Status,
    response::status,
};

use crate::database::{
    self,
    models::product::{Product, CreateProduct, UpdateProduct},
    schema::product::dsl::*,
};

#[post("/", data = "<_product>")]
pub fn input(_product: Json<CreateProduct>) -> Json<Product> {
    use crate::database::schema::product;

    let connection = &mut database::establish_connection();
    diesel::insert_into(product::table)
        .values(_product.into_inner())
        .execute(connection)
        .expect("Error adding sighting");

    Json(product::table
        .order(product::id.desc())
        .first(connection).unwrap()
    )
}

#[get("/")]
pub fn get() -> Json<Vec<Product>> {
    let connection = &mut database::establish_connection();
    product.load::<Product>(connection)
        .map(Json)
        .expect("Error loading birds")
}

#[get("/<_id>")]
pub fn get_one(_id: i32) -> Result<Json<Product>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = product
         .filter(id.eq(_id))
         .first::<Product>(connection);

    match result {
        Ok(found_product) => Ok(Json(found_product)),
        Err(_) => Err(status::Custom(Status::NotFound, "Product not found".to_string())),
    }
}

#[patch("/<_id>", data = "<patch_product>")]
pub fn update(_id: i32, patch_product: Json<UpdateProduct>) -> Result<Json<Product>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    
    let update_result = diesel::update(product.filter(id.eq(_id)))
        .set(&patch_product.into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => {
            match product.filter(id.eq(_id)).first::<Product>(connection) {
                Ok(updated_product) => Ok(Json(updated_product)),
                Err(_) => Err(status::Custom(Status::NotFound, "Product not found after update".to_string())),
            }
        },
        Err(_) => Err(status::Custom(Status::InternalServerError, "Error updating product".to_string())),
    }
}

#[delete("/<_id>")]
pub fn delete(_id: i32) -> Result<Json<Product>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = product
         .filter(id.eq(_id))
         .first::<Product>(connection);

    match result {
        Ok(found_product) => {
            diesel::delete(product.filter(id.eq(_id)))
                .execute(connection).expect("Error deleting sighting");
            Ok(Json(found_product))
        },
        Err(_) => Err(status::Custom(Status::NotFound, "Product not found".to_string())),
    }
}
