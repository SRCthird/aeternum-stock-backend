use crate::database::{self, models::inventory::Inventory, schema::inventory::dsl};
use diesel::prelude::*;

use rocket::{http::Status, response::status, serde::json::Json};
#[get("/?<location>")]
pub fn get(
    location: Option<String>,
) ->Result<Json<Vec<Inventory>>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let mut query = dsl::inventory.into_boxed();

    if let Some(location) = location {
        query = query.filter(dsl::location.like(format!("{}%", location)));
    }
    let result = query.load(connection);

    match result {
        Ok(inventorys) => Ok(Json(inventorys)),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error getting inventorys".to_string(),
        )),
    }
}

#[get("/count?<location>")]
pub fn get_count(
    location: Option<String>,
) -> Result<Json<i64>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let mut query = dsl::inventory.into_boxed();

    if let Some(location) = location {
        query = query.filter(dsl::location.like(format!("{}%", location)));
    }

    let result = query
        .count()
        .get_result::<i64>(connection);

    match result {
        Ok(count) => Ok(Json(count)),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error getting inventory count".to_string(),
        )),
    }
}
