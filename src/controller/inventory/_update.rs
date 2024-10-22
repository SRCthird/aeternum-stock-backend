use crate::controller::inventory::utils::merge_lot;
use crate::database::{
    self,
    models::{
        inventory::{CreateInventory, Inventory, UpdateInventory},
        log::CreateLog,
        productlot::ProductLot,
    },
    schema::{inventory::dsl, inventorybay, log, productlot},
};
use diesel::{
    dsl::{not, sql},
    prelude::*,
    result::{
        DatabaseErrorKind::{ForeignKeyViolation, UniqueViolation},
        Error::{DatabaseError, NotFound},
    },
    sql_types::BigInt,
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

    let lot: ProductLot;
    match &patch_inventory.lot_number {
        Some(lot_number) => {
            lot = productlot::dsl::productlot
                .filter(productlot::dsl::lot_number.eq(lot_number))
                .get_result::<ProductLot>(connection)
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
        }
        None => {
            lot = productlot::dsl::productlot
                .filter(productlot::dsl::lot_number.eq(&found_inventory.lot_number))
                .get_result::<ProductLot>(connection)
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
        }
    }

    if let Some(location) = &patch_inventory.location {
        println!("{:?}", &patch_inventory);
        let bay_max_count = inventorybay::dsl::inventorybay
            .select(sql::<BigInt>("SUM(max_unique_lots)"))
            .filter(inventorybay::dsl::name.eq(location))
            .get_result(connection)
            .unwrap_or(0);
        println!("{:?}", &bay_max_count);

        if bay_max_count == 0 {
            return Err(status::Custom(
                    Status::PayloadTooLarge,
                    "Location not found in system, or not a storable location".to_string(),
            ));
        }

        let unique_lots: i64 = dsl::inventory
            .select(sql::<BigInt>("COUNT(DISTINCT lot_number)"))
            .filter(not(dsl::lot_number.eq(&found_inventory.lot_number)))
            .filter(dsl::location.eq(&location))
            .get_result(connection)
            .unwrap_or(0);
        println!("{:?}", &unique_lots);
        if unique_lots >= bay_max_count {
            return Err(status::Custom(
                    Status::Conflict,
                    "Inventory already at capacity".to_string(),
            ));
        }
    }

    if let Some(quantity) = patch_inventory.quantity {
        if quantity > lot.quantity {
            return Err(status::Custom(
                Status::UnprocessableEntity,
                "Inventory quantity exceeds product lot quantity".to_string(),
            ));
        }
    }

    let updated_inventory = CreateInventory {
        lot_number: patch_inventory
            .lot_number
            .clone()
            .unwrap_or(found_inventory.lot_number.clone()),
        location: patch_inventory
            .location
            .clone()
            .unwrap_or(found_inventory.location.clone()),
        quantity: patch_inventory
            .quantity
            .clone()
            .unwrap_or(found_inventory.quantity.clone()),
        created_by: patch_inventory
            .updated_by
            .clone()
            .unwrap_or(found_inventory.created_by.clone()),
        created_at: Some(
            patch_inventory
                .updated_at
                .clone()
                .unwrap_or(chrono::Utc::now().naive_utc()),
        ),
        from_location: patch_inventory
            .from_location
            .clone()
            .unwrap_or(found_inventory.from_location.clone()),
        comments: patch_inventory
            .comments
            .clone()
            .unwrap_or(found_inventory.comments.clone()),
    };

    let merged = merge_lot(updated_inventory.clone(), Some(id));

    match merged {
        Ok(lot) => {
            return {
                diesel::delete(dsl::inventory.filter(dsl::id.eq(id)))
                    .execute(connection)
                    .map_err(|_| {
                        status::Custom(
                            Status::InternalServerError,
                            "Error deleting inventory".to_string(),
                        )
                    })?;
                Ok(Json(lot))
            }
        }
        Err(e) => {
            if e.0 != Status::NotFound {
                return Err(status::Custom(
                    Status::InternalServerError,
                    "An error occurred while merging the lot".to_string(),
                ));
            }
        }
    }

    let update_result = diesel::update(dsl::inventory.filter(dsl::id.eq(id)))
        .set(&patch_inventory.clone().into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => {
            if let Some(from_location) = &patch_inventory.from_location {
                diesel::insert_into(log::dsl::log)
                    .values(CreateLog {
                        from_location: from_location.clone(),
                        to_location: patch_inventory
                            .location
                            .clone()
                            .unwrap_or(found_inventory.location),
                        user: patch_inventory
                            .updated_by
                            .clone()
                            .unwrap_or(found_inventory.created_by),
                        lot_number: patch_inventory
                            .lot_number
                            .clone()
                            .unwrap_or(found_inventory.lot_number),
                        quantity_moved: patch_inventory
                            .quantity
                            .clone()
                            .unwrap_or(found_inventory.quantity),
                        comments: patch_inventory
                            .comments
                            .clone()
                            .unwrap_or(found_inventory.comments),
                    })
                    .execute(connection)
                    .map_err(|e| {
                        status::Custom(
                            Status::InternalServerError,
                            format!("Error inserting log: {:?}", e),
                        )
                    })?;
            }
            let updated_inventory = dsl::inventory
                .filter(dsl::id.eq(id))
                .first::<Inventory>(connection)
                .map_err(|e| match e {
                    NotFound => {
                        status::Custom(Status::NotFound, "Updated inventory not found".to_string())
                    }
                    _ => status::Custom(
                        Status::InternalServerError,
                        "Error loading updated inventory".to_string(),
                    ),
                })?;
            Ok(Json(updated_inventory))
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
