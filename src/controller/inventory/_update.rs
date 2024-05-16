use crate::database::{
    self,
    models::inventory::{Inventory, UpdateInventory},
    schema::inventory::dsl,
};
use diesel::{
    prelude::*,
    result::{
        DatabaseErrorKind::{ForeignKeyViolation, UniqueViolation},
        Error::{DatabaseError, NotFound},
    },
};
use rocket::{http::Status, response::status, serde::json::Json};

#[patch("/<id>", data = "<patch_inventory>")]
pub fn update(
    id: i32,
    patch_inventory: Json<UpdateInventory>,
) -> Result<Json<Inventory>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    let found_inventory = dsl::inventory
        .filter(dsl::id.eq(id))
        .first::<Inventory>(connection)
        .map_err(|e| match e {
            NotFound => status::Custom(
                Status::NotFound,
                "Inventory not found in system".to_string(),
            ),
            _ => status::Custom(
                Status::InternalServerError,
                "Error loading inventory".to_string(),
            ),
        })?;

    let update_result = diesel::update(dsl::inventory.filter(dsl::id.eq(id)))
        .set(&patch_inventory.into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => Ok(Json(found_inventory)),
        Err(e) => Err(match e {
            NotFound => status::Custom(Status::NotFound, "Inventory not found".to_string()),
            DatabaseError(UniqueViolation, _) => status::Custom(
                Status::Conflict,
                "Inventory already exists in system".to_string(),
            ),
            DatabaseError(ForeignKeyViolation, _) => {
                status::Custom(Status::Conflict, "Inventory does not exist".to_string())
            }
            _ => status::Custom(
                Status::InternalServerError,
                "Error inserting inventory".to_string(),
            ),
        }),
    }
}
