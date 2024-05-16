use crate::database::{
    self,
    models::product::{CreateProduct, UpdateProduct, Product},
    schema::product::{dsl, table},
};
use diesel::{prelude::*, result::{
    Error::{DatabaseError, NotFound},
    DatabaseErrorKind::{UniqueViolation, ForeignKeyViolation},
}};
use crate::controller::utils;
use rocket::{http::Status, response::status, serde::json::Json};

#[post("/", data = "<product>")]
pub fn input(product: Json<CreateProduct>) -> Result<Json<Product>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    let result = diesel::insert_into(table)
        .values(product.into_inner())
        .execute(connection);

    match result {
        Ok(_) => {
            let inserted_product = table.order(dsl::id.desc()).first(connection).unwrap();
            Ok(Json(inserted_product))
        }
        Err(e) => Err(match e {
            NotFound => status::Custom(Status::NotFound, "Product not found".to_string()),
            DatabaseError(UniqueViolation, _) => status::Custom(
                Status::Conflict,
                "Product already exists in system".to_string(),
            ),
            DatabaseError(ForeignKeyViolation, _) => status::Custom(
                Status::Conflict,
                "Product does not exist".to_string(),
            ),
            _ => status::Custom(
                Status::InternalServerError,
                "Error inserting product".to_string(),
            ),
        }),
    }
}

#[get("/list")]
pub fn list() -> Result<Json<Vec<String>>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = dsl::product.select(dsl::name).load::<String>(connection);

    match result {
        Ok(products) => Ok(Json(products)),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error loading products".to_string(),
        )),
    }
}

#[get("/?<name>&<description>")]
pub fn get(name: Option<String>, description: Option<String>) -> Result<Json<Vec<Product>>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    match utils::validate_string(&name)
        && utils::validate_string(&description)
    {
        true => (),
        false => return Err(status::Custom(Status::BadRequest, "Invalid character in search query".to_string())),
    }

    let query_result: QueryResult<Vec<Product>> = 
        match (name, description) {
            (Some(name), Some(description)) => dsl::product
                .filter(dsl::name.like(format!("{}%", name)))
                .filter(dsl::description.like(format!("{}%", description)))
                .load(connection),
            (Some(name), None) => dsl::product
                .filter(dsl::name.like(format!("{}%", name)))
                .load(connection),
            (None, Some(description)) => dsl::product
                .filter(dsl::description.like(format!("{}%", description)))
                .load(connection),
            (None, None) => dsl::product.load(connection),
        };

    match query_result {
        Ok(products) => Ok(Json(products)),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error getting products".to_string(),
        )),
    }
}

#[get("/<id>")]
pub fn get_one(id: i32) -> Result<Json<Product>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = dsl::product
        .filter(dsl::id.eq(id))
        .first::<Product>(connection);

    match result {
        Ok(found_product) => Ok(Json(found_product)),
        Err(_) => Err(status::Custom(
            Status::NotFound,
            "Product not found".to_string(),
        )),
    }
}

#[patch("/<id>", data = "<patch_product>")]
pub fn update(
    id: i32,
    patch_product: Json<UpdateProduct>,
) -> Result<Json<Product>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    let found_product = dsl::product
        .filter(dsl::id.eq(id))
        .first::<Product>(connection)
        .map_err(|e| match e {
            NotFound => status::Custom(
                Status::NotFound,
                "Product not found in system".to_string(),
            ),
            _ => status::Custom(
                Status::InternalServerError,
                "Error loading product".to_string(),
            ),
        })?;

    let update_result = diesel::update(dsl::product.filter(dsl::id.eq(id)))
        .set(&patch_product.into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => Ok(Json(found_product)),
        Err(e) => Err(match e {
            NotFound => status::Custom(Status::NotFound, "Product not found".to_string()),
            DatabaseError(UniqueViolation, _) => status::Custom(
                Status::Conflict,
                "Product already exists in system".to_string(),
            ),
            DatabaseError(ForeignKeyViolation, _) => status::Custom(
                Status::Conflict,
                "Product does not exist".to_string(),
            ),
            _ => status::Custom(
                Status::InternalServerError,
                "Error inserting product".to_string(),
            ),
        }),
    }
}

#[delete("/<id>")]
pub fn delete(id: i32) -> Result<Json<Product>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let found_product = dsl::product
        .filter(dsl::id.eq(id))
        .first::<Product>(connection)
        .map_err(|e| match e {
            NotFound => status::Custom(
                Status::NotFound,
                "Product not found in system".to_string(),
            ),
            _ => status::Custom(
                Status::InternalServerError,
                "Error loading product".to_string(),
            ),
        })?;

    let result = diesel::delete(dsl::product.filter(dsl::id.eq(id))).execute(connection);

    match result {
        Ok(_) => Ok(Json(found_product)),
        Err(_) => Err(status::Custom(
            Status::NotFound,
            "Product not found".to_string(),
        )),
    }
}
