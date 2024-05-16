use crate::database::{
    self,
    models::inventorybay::{CreateInventoryBay, InventoryBay, UpdateInventoryBay},
    schema::inventorybay::{dsl, table},
};
use crate::controller::utils;
use diesel::{
    prelude::*,
    result::{
        DatabaseErrorKind::{ForeignKeyViolation, UniqueViolation},
        Error::{DatabaseError, NotFound},
    },
};
use rocket::{http::Status, response::status, serde::json::Json};

#[post("/", data = "<inventorybay>")]
pub fn input(inventorybay: Json<CreateInventoryBay>) -> Result<Json<InventoryBay>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    let result = diesel::insert_into(table)
        .values(inventorybay.into_inner())
        .execute(connection);

    match result {
        Ok(_) => {
            let inserted_inventorybay = table.order(dsl::id.desc()).first(connection).unwrap();
            Ok(Json(inserted_inventorybay))
        }
        Err(e) => Err(match e {
            DatabaseError(UniqueViolation, _) => status::Custom(
                Status::Conflict,
                "Bay already exists in system".to_string(),
            ),
            DatabaseError(ForeignKeyViolation, _) => status::Custom(
                Status::Conflict,
                "Warehouse does not exist".to_string(),
            ),
            _ => status::Custom(
                Status::InternalServerError,
                "Error inserting product".to_string(),
            ),
        }),
    }
}

#[get("/?<name>&<warehouse_name>&<max_unique_lots>")]
pub fn get(
    name: Option<String>,
    warehouse_name: Option<String>,
    max_unique_lots: Option<i32>,
) -> Result<Json<Vec<InventoryBay>>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    match utils::validate_string(&name)
        && utils::validate_string(&warehouse_name)
    {
        false => {
            return Err(status::Custom(
                Status::BadRequest,
                "Invalid character input".to_string(),
            ))
        }
        _ => (),
    }

    let query_result: QueryResult<Vec<InventoryBay>> =
        match (name, warehouse_name, max_unique_lots) {
            (Some(name), Some(warehouse_name), Some(max_unique_lots)) => dsl::inventorybay
                .filter(dsl::name.like(format!("{}%", name)))
                .filter(dsl::warehouse_name.like(format!("{}%", warehouse_name)))
                .filter(dsl::max_unique_lots.eq(max_unique_lots))
                .load(connection),
            (Some(name), Some(warehouse_name), None) => dsl::inventorybay
                .filter(dsl::name.like(format!("{}%", name)))
                .filter(dsl::warehouse_name.like(format!("{}%", warehouse_name)))
                .load(connection),
            (Some(name), None, Some(max_unique_lots)) => dsl::inventorybay
                .filter(dsl::name.like(format!("{}%", name)))
                .filter(dsl::max_unique_lots.eq(max_unique_lots))
                .load(connection),
            (None, Some(warehouse_name), Some(max_unique_lots)) => dsl::inventorybay
                .filter(dsl::warehouse_name.like(format!("{}%", warehouse_name)))
                .filter(dsl::max_unique_lots.eq(max_unique_lots))
                .load(connection),
            (Some(name), None, None) => dsl::inventorybay
                .filter(dsl::name.like(format!("{}%", name)))
                .load(connection),
            (None, Some(warehouse_name), None) => dsl::inventorybay
                .filter(dsl::warehouse_name.like(format!("{}%", warehouse_name)))
                .load(connection),
            (None, None, Some(max_unique_lots)) => dsl::inventorybay
                .filter(dsl::max_unique_lots.eq(max_unique_lots))
                .load(connection),
            (None, None, None) => dsl::inventorybay.load(connection),
        };

    match query_result {
        Ok(inventorybays) => Ok(Json(inventorybays)),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error getting product lots".to_string(),
        )),
    }
}

#[get("/<id>")]
pub fn get_one(id: i32) -> Result<Json<InventoryBay>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = dsl::inventorybay.filter(dsl::id.eq(id)).first::<InventoryBay>(connection);

    match result {
        Ok(found_inventorybay) => Ok(Json(found_inventorybay)),
        Err(_) => Err(status::Custom(
            Status::NotFound,
            "InventoryBay not found".to_string(),
        )),
    }
}

#[patch("/<id>", data = "<patch_inventorybay>")]
pub fn update(id: i32, patch_inventorybay: Json<UpdateInventoryBay>) -> Result<Json<InventoryBay>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    let found_inventorybay = dsl::inventorybay
        .filter(dsl::id.eq(id))
        .first::<InventoryBay>(connection)
        .map_err(|e| match e {
            NotFound => status::Custom(Status::NotFound, "InventoryBay not found in system".to_string()),
            _ => status::Custom(Status::InternalServerError, "Error loading inventorybay".to_string()),
        })?;

    let update_result = diesel::update(dsl::inventorybay.filter(dsl::id.eq(id)))
        .set(&patch_inventorybay.into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => Ok(Json(found_inventorybay)),
        Err(e) => Err(match e {
            DatabaseError(UniqueViolation, _) => status::Custom(
                Status::Conflict,
                "Bay already exists in system".to_string(),
            ),
            DatabaseError(ForeignKeyViolation, _) => status::Custom(
                Status::Conflict,
                "Warehouse does not exist".to_string(),
            ),
            _ => status::Custom(
                Status::InternalServerError,
                "Error inserting product".to_string(),
            ),
        }),
    }
}

#[delete("/<id>")]
pub fn delete(id: i32) -> Result<Json<InventoryBay>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let found_inventorybay = dsl::inventorybay
        .filter(dsl::id.eq(id))
        .first::<InventoryBay>(connection)
        .map_err(|e| match e {
            NotFound => status::Custom(Status::NotFound, "InventoryBay not found in system".to_string()),
            _ => status::Custom(Status::InternalServerError, "Error loading inventorybay".to_string()),
        })?;

    let result = diesel::delete(dsl::inventorybay.filter(dsl::id.eq(id))).execute(connection);

    match result {
        Ok(_) => Ok(Json(found_inventorybay)),
        Err(e) => Err(match e {
            DatabaseError(ForeignKeyViolation, _) => status::Custom(
                Status::Conflict,
                "InventoryBay is in use".to_string(),
            ),
            _ => status::Custom(
                Status::InternalServerError,
                "Error deleting inventorybay".to_string(),
            ),
        }),
    }
}
