use crate::database::schema::{inventory::dsl, log};
use crate::database::{
    self,
    models::{
        inventory::{CreateInventory, Inventory},
        log::CreateLog,
    },
};
use diesel::prelude::*;
use diesel::result::Error::NotFound;
use rocket::{http::Status, response::status};

pub fn merge_lot(
    inventory: CreateInventory,
) -> Result<Inventory, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let mergable_lot = dsl::inventory
        .filter(dsl::lot_number.eq(&inventory.lot_number))
        .filter(dsl::location.eq(&inventory.location))
        .first::<Inventory>(connection)
        .map_err(|e| match e {
            NotFound => status::Custom(Status::NotFound, "Inventory not found".to_string()),
            _ => status::Custom(
                Status::InternalServerError,
                format!("Error loading inventory: {:?}", e),
            ),
        })?;

    diesel::update(dsl::inventory)
        .filter(dsl::id.eq(mergable_lot.id))
        .set(dsl::quantity.eq(mergable_lot.quantity + inventory.quantity))
        .execute(connection)
        .map_err(|e| {
            status::Custom(
                Status::InternalServerError,
                format!("Error updating inventory: {:?}", e),
            )
        })?;

    let log = CreateLog {
        from_location: inventory.from_location.clone(),
        to_location: inventory.location.clone(),
        user: inventory.created_by.clone(),
        lot_number: inventory.lot_number.clone(),
        quantity_moved: inventory.quantity,
        comments: inventory.comments.clone(),
    };
    diesel::insert_into(log::dsl::log)
        .values(log)
        .execute(connection)
        .map_err(|e| {
            status::Custom(
                Status::InternalServerError,
                format!("Error inserting log: {:?}", e),
            )
        })?;

    let updated_inventory = dsl::inventory
        .filter(dsl::id.eq(mergable_lot.id))
        .first::<Inventory>(connection)
        .map_err(|e| match e {
            NotFound => status::Custom(Status::NotFound, "Updated inventory not found".to_string()),
            _ => status::Custom(
                Status::InternalServerError,
                format!("Error loading updated inventory: {:?}", e),
            ),
        })?;

    Ok(updated_inventory)
}
