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
            let inserted_inventorybay = table
                .order(dsl::id.desc())
                .first(connection).unwrap();
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

#[get("/list")]
pub fn list() -> Result<Json<Vec<String>>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = dsl::inventorybay.select(dsl::name).load::<String>(connection);

    match result {
        Ok(bay) => Ok(Json(bay)),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error loading bays".to_string(),
        )),
    }
}

#[get("/?<name>&<friendly_name>&<warehouse_name>&<max_unique_lots>")]
pub fn get(
    name: Option<String>,
    friendly_name: Option<String>,
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

    
    let mut query = dsl::inventorybay.into_boxed();
    
    if let Some(ref name) = name {
        query = query.filter(dsl::name.like(format!("{}%", name)));
    }

    if let Some(ref friendly_name) = friendly_name {
        query = query.filter(dsl::friendly_name.like(format!("{}%", friendly_name)));
    }

    if let Some(ref warehouse_name) = warehouse_name {
        query = query.filter(dsl::warehouse_name.like(format!("{}%", warehouse_name)));
    }

    if let Some(max_unique_lots) = max_unique_lots {
        query = query.filter(dsl::max_unique_lots.eq(max_unique_lots));
    }

    let query_result: QueryResult<Vec<InventoryBay>> = query.load(connection);

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
