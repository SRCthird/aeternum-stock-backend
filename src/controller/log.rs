use diesel::prelude::*;
use rocket::{
    serde::json::Json,
    http::Status,
    response::status,
};

use crate::database::{
    self,
    models::log::{Log, CreateLog, UpdateLog},
    schema::log::dsl::*,
};

#[post("/", data = "<_log>")]
pub fn input(_log: Json<CreateLog>) -> Json<Log> {
    use crate::database::schema::log;

    let connection = &mut database::establish_connection();
    diesel::insert_into(log::table)
        .values(_log.into_inner())
        .execute(connection)
        .expect("Error adding sighting");

    Json(log::table
        .order(log::id.desc())
        .first(connection).unwrap()
    )
}

#[get("/")]
pub fn get() -> Json<Vec<Log>> {
    let connection = &mut database::establish_connection();
    log.load::<Log>(connection)
        .map(Json)
        .expect("Error loading birds")
}

#[get("/<_id>")]
pub fn get_one(_id: i32) -> Result<Json<Log>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = log
         .filter(id.eq(_id))
         .first::<Log>(connection);

    match result {
        Ok(found_log) => Ok(Json(found_log)),
        Err(_) => Err(status::Custom(Status::NotFound, "Log not found".to_string())),
    }
}

#[patch("/<_id>", data = "<patch_log>")]
pub fn update(_id: i32, patch_log: Json<UpdateLog>) -> Result<Json<Log>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    
    let update_result = diesel::update(log.filter(id.eq(_id)))
        .set(&patch_log.into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => {
            match log.filter(id.eq(_id)).first::<Log>(connection) {
                Ok(updated_log) => Ok(Json(updated_log)),
                Err(_) => Err(status::Custom(Status::NotFound, "Log not found after update".to_string())),
            }
        },
        Err(_) => Err(status::Custom(Status::InternalServerError, "Error updating log".to_string())),
    }
}

#[delete("/<_id>")]
pub fn delete(_id: i32) -> Result<Json<Log>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = log
         .filter(id.eq(_id))
         .first::<Log>(connection);

    match result {
        Ok(found_log) => {
            diesel::delete(log.filter(id.eq(_id)))
                .execute(connection).expect("Error deleting sighting");
            Ok(Json(found_log))
        },
        Err(_) => Err(status::Custom(Status::NotFound, "Log not found".to_string())),
    }
}
