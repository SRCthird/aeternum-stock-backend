use crate::controller::utils;
use crate::database::{
    self,
    models::productlot::{CreateProductLot, ProductLot, UpdateProductLot},
    schema::productlot::{dsl, table},
};
use diesel::{
    prelude::*,
    result::{
        DatabaseErrorKind::{ForeignKeyViolation, UniqueViolation},
        Error::{DatabaseError, NotFound},
    },
};
use rocket::{http::Status, response::status, serde::json::Json};

#[post("/", data = "<productlot>")]
pub fn input(
    productlot: Json<CreateProductLot>,
) -> Result<Json<ProductLot>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    let result = diesel::insert_into(table)
        .values(productlot.into_inner())
        .execute(connection);

    match result {
        Ok(_) => {
            let inserted_productlot = table.order(dsl::id.desc()).first(connection).unwrap();
            Ok(Json(inserted_productlot))
        }
        Err(e) => Err(match e {
            NotFound => status::Custom(Status::NotFound, "Product not found".to_string()),
            DatabaseError(ForeignKeyViolation, _) => {
                status::Custom(Status::NotFound, "Product does not exist".to_string())
            }
            DatabaseError(UniqueViolation, _) => status::Custom(
                Status::Conflict,
                "Product lot or workorder already exists".to_string(),
            ),
            _ => {
                println!("{:?}", e);
                status::Custom(
                    Status::InternalServerError,
                    "Error creating product lot".to_string(),
                )
            }
        }),
    }
}

#[get("/?<lot_number>&<internal_reference>&<product_name>")]
pub fn get(
    lot_number: Option<String>,
    internal_reference: Option<String>,
    product_name: Option<String>,
) -> Result<Json<Vec<ProductLot>>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    match utils::validate_string(&lot_number)
        && utils::validate_string(&internal_reference)
        && utils::validate_string(&product_name)
    {
        false => {
            return Err(status::Custom(
                Status::BadRequest,
                "Invalid character input".to_string(),
            ))
        }
        _ => (),
    }

    let query_result: QueryResult<Vec<ProductLot>> =
        match (lot_number, internal_reference, product_name) {
            (Some(lot_number), Some(internal_reference), Some(product_name)) => dsl::productlot
                .filter(dsl::lot_number.like(format!("{}%", lot_number)))
                .filter(dsl::internal_reference.like(format!("{}%", internal_reference)))
                .filter(dsl::product_name.like(format!("{}%", product_name)))
                .load(connection),
            (Some(lot_number), Some(internal_reference), None) => dsl::productlot
                .filter(dsl::lot_number.like(format!("{}%", lot_number)))
                .filter(dsl::internal_reference.like(format!("{}%", internal_reference)))
                .load(connection),
            (Some(lot_number), None, Some(product_name)) => dsl::productlot
                .filter(dsl::lot_number.like(format!("{}%", lot_number)))
                .filter(dsl::product_name.like(format!("{}%", product_name)))
                .load(connection),
            (None, Some(internal_reference), Some(product_name)) => dsl::productlot
                .filter(dsl::internal_reference.like(format!("{}%", internal_reference)))
                .filter(dsl::product_name.like(format!("{}%", product_name)))
                .load(connection),
            (Some(lot_number), None, None) => dsl::productlot
                .filter(dsl::lot_number.like(format!("{}%", lot_number)))
                .load(connection),
            (None, Some(internal_reference), None) => dsl::productlot
                .filter(dsl::internal_reference.like(format!("{}%", internal_reference)))
                .load(connection),
            (None, None, Some(product_name)) => dsl::productlot
                .filter(dsl::product_name.like(format!("{}%", product_name)))
                .load(connection),
            (None, None, None) => dsl::productlot.load(connection),
        };

    match query_result {
        Ok(productlots) => Ok(Json(productlots)),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error getting product lots".to_string(),
        )),
    }
}

#[get("/list")]
pub fn list() -> Result<Json<Vec<String>>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = dsl::productlot
        .select(dsl::lot_number)
        .load::<String>(connection);

    match result {
        Ok(productlots) => Ok(Json(productlots)),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error loading productlots".to_string(),
        )),
    }
}

#[get("/<id>")]
pub fn get_one(id: i32) -> Result<Json<ProductLot>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = dsl::productlot
        .filter(dsl::id.eq(id))
        .first::<ProductLot>(connection);

    match result {
        Ok(found_productlot) => Ok(Json(found_productlot)),
        Err(_) => Err(status::Custom(
            Status::NotFound,
            "Product Lot not found".to_string(),
        )),
    }
}

#[patch("/<id>", data = "<patch_productlot>")]
pub fn update(
    id: i32,
    patch_productlot: Json<UpdateProductLot>,
) -> Result<Json<ProductLot>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    let found_productlot = dsl::productlot
        .filter(dsl::id.eq(id))
        .first::<ProductLot>(connection)
        .map_err(|e| match e {
            NotFound => status::Custom(
                Status::NotFound,
                "Product Lot not found in system".to_string(),
            ),
            _ => status::Custom(
                Status::InternalServerError,
                "Error loading product lot".to_string(),
            ),
        })?;

    let update_result = diesel::update(dsl::productlot.filter(dsl::id.eq(id)))
        .set(&patch_productlot.into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => Ok(Json(found_productlot)),
        Err(e) => Err(match e {
            NotFound => status::Custom(Status::NotFound, "Product not found".to_string()),
            DatabaseError(ForeignKeyViolation, _) => {
                status::Custom(Status::NotFound, "Product does not exist".to_string())
            }
            DatabaseError(UniqueViolation, _) => status::Custom(
                Status::Conflict,
                "Product lot or workorder already exists".to_string(),
            ),
            _ => {
                println!("{:?}", e);
                status::Custom(
                    Status::InternalServerError,
                    "Error creating product lot".to_string(),
                )
            }
        }),
    }
}

#[delete("/<id>")]
pub fn delete(id: i32) -> Result<Json<ProductLot>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let found_productlot = dsl::productlot
        .filter(dsl::id.eq(id))
        .first::<ProductLot>(connection)
        .map_err(|e| match e {
            NotFound => status::Custom(
                Status::NotFound,
                "Product Lot not found in system".to_string(),
            ),
            _ => status::Custom(
                Status::InternalServerError,
                "Error loading product lot".to_string(),
            ),
        })?;

    let result = diesel::delete(dsl::productlot.filter(dsl::id.eq(id))).execute(connection);

    match result {
        Ok(_) => Ok(Json(found_productlot)),
        Err(e) => Err(match e {
            NotFound => status::Custom(Status::NotFound, "Product not found".to_string()),
            DatabaseError(ForeignKeyViolation, _) => {
                status::Custom(Status::Conflict, "Lot has dependencies".to_string())
            }
            _ => status::Custom(
                Status::InternalServerError,
                "Error deleting product lot".to_string(),
            ),
        }),
    }
}
