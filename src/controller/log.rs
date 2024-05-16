use crate::database::{
    self,
    models::log::{CreateLog, Log, UpdateLog},
    schema::log::{dsl, table},
};
use diesel::{prelude::*, result::Error::NotFound};
use rocket::{http::Status, response::status, serde::json::Json};

#[post("/", data = "<log>")]
pub fn input(log: Json<CreateLog>) -> Result<Json<Log>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    let result = diesel::insert_into(table)
        .values(log.into_inner())
        .execute(connection);

    match result {
        Ok(_) => {
            let inserted_log = table.order(dsl::id.desc()).first(connection).unwrap();
            Ok(Json(inserted_log))
        }
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error inserting log".to_string(),
        )),
    }
}

#[get("/")]
pub fn get() -> Result<Json<Vec<Log>>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let logs = dsl::log.load(connection);

    match logs {
        Ok(logs) => Ok(Json(logs)),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error getting logs".to_string(),
        )),
    }
}

#[get("/<id>")]
pub fn get_one(id: i32) -> Result<Json<Log>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = dsl::log.filter(dsl::id.eq(id)).first::<Log>(connection);

    match result {
        Ok(found_log) => Ok(Json(found_log)),
        Err(_) => Err(status::Custom(
            Status::NotFound,
            "Log not found".to_string(),
        )),
    }
}

#[patch("/<id>", data = "<patch_log>")]
pub fn update(id: i32, patch_log: Json<UpdateLog>) -> Result<Json<Log>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    let found_log = dsl::log
        .filter(dsl::id.eq(id))
        .first::<Log>(connection)
        .map_err(|e| match e {
            NotFound => status::Custom(Status::NotFound, "Log not found in system".to_string()),
            _ => status::Custom(Status::InternalServerError, "Error loading log".to_string()),
        })?;

    let update_result = diesel::update(dsl::log.filter(dsl::id.eq(id)))
        .set(&patch_log.into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => Ok(Json(found_log)),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error updating log".to_string(),
        )),
    }
}

#[delete("/<id>")]
pub fn delete(id: i32) -> Result<Json<Log>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let found_log = dsl::log
        .filter(dsl::id.eq(id))
        .first::<Log>(connection)
        .map_err(|e| match e {
            NotFound => status::Custom(Status::NotFound, "Log not found in system".to_string()),
            _ => status::Custom(Status::InternalServerError, "Error loading log".to_string()),
        })?;

    let result = diesel::delete(dsl::log.filter(dsl::id.eq(id))).execute(connection);

    match result {
        Ok(_) => Ok(Json(found_log)),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error deleting log".to_string(),
        )),
    }
}
