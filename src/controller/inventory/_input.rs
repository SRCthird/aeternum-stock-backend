use crate::controller::inventory::utils::merge_lot;
use crate::database::{
    self,
    models::{
        inventory::{CreateInventory, Inventory},
        productlot::ProductLot,
    },
    schema::{
        inventory::{dsl, table},
        inventorybay, log, productlot,
    },
};
use diesel::{
    dsl::sql,
    prelude::*,
    result::{
        DatabaseErrorKind::{ForeignKeyViolation, UniqueViolation},
        Error::{DatabaseError, NotFound},
    },
    sql_types::BigInt,
};

use rocket::{http::Status, response::status, serde::json::Json};
#[post("/", data = "<inventory>")]
pub fn input(inventory: Json<CreateInventory>) -> Result<Json<Inventory>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let lot = productlot::dsl::productlot
        .filter(productlot::dsl::lot_number.eq(&inventory.lot_number))
        .first::<ProductLot>(connection)
        .map_err(|e| match e {
            NotFound => status::Custom(
                Status::NotFound,
                "Product lot not found in system".to_string(),
            ),
            _ => status::Custom(
                Status::InternalServerError,
                "Error loading product lot".to_string(),
            ),
        })?;

    let bay_max_count = inventorybay::dsl::inventorybay
        .select(sql::<BigInt>("SUM(max_unique_lots)"))
        .filter(inventorybay::dsl::name.eq(&inventory.location))
        .get_result(connection)
        .unwrap_or(0);

    if bay_max_count == 0 {
        return Err(status::Custom(
            Status::NotFound,
            "Location not found in system, or not a storable location".to_string(),
        ));
    }

    let lot_total: i64 = dsl::inventory
        .filter(dsl::lot_number.eq(&inventory.lot_number))
        .select(sql::<BigInt>("SUM(quantity)"))
        .get_result(connection)
        .unwrap_or(0);
    {
        let lot_total_i32: i32 = lot_total.try_into().map_err(|_| {
            status::Custom(
                Status::InternalServerError,
                "Failed to convert lot total to i32".to_string(),
            )
        })?;

        if lot_total_i32 + inventory.quantity > lot.quantity {
            return Err(status::Custom(
                Status::Conflict,
                "Inventory quantity exceeds product lot quantity".to_string(),
            ));
        }
    }

    let unique_lots: i64 = dsl::inventory
        .filter(dsl::lot_number.eq(&inventory.lot_number))
        .filter(dsl::location.eq(&inventory.location))
        .select(sql::<BigInt>("COUNT(DISTINCT lot_number)"))
        .get_result(connection)
        .unwrap_or(0);
    if unique_lots >= bay_max_count {
        return Err(status::Custom(
            Status::Conflict,
            "Inventory already exists in system".to_string(),
        ));
    }

    let merged = merge_lot(inventory.clone().into_inner());

    match merged {
        Ok(lot) => return Ok(Json(lot)),
        Err(e) => {
            if e.0 != Status::NotFound {
                return Err(status::Custom(
                    Status::InternalServerError,
                    "An error occurred while merging the lot".to_string(),
                ));
            }
        }
    }

    let result = diesel::insert_into(table)
        .values(inventory.clone().into_inner())
        .execute(connection);

    match result {
        Ok(_) => {
            let inserted_inventory = table.order(dsl::id.desc()).first(connection).unwrap();
            diesel::insert_into(log::dsl::log)
                .values((
                    log::dsl::from_location.eq(&inventory.from_location),
                    log::dsl::to_location.eq(&inventory.location),
                    log::dsl::user.eq(&inventory.created_by),
                    log::dsl::lot_number.eq(&inventory.lot_number),
                    log::dsl::quantity_moved.eq(&inventory.quantity),
                    log::dsl::comments.eq(&inventory.comments),
                ))
                .execute(connection)
                .map_err(|e| {
                    status::Custom(
                        Status::InternalServerError,
                        format!("Error inserting log: {:?}", e),
                    )
                })?;
            Ok(Json(inserted_inventory))
        }
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
