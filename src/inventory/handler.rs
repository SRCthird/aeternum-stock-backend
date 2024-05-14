use axum::Json;
use crate::database::{
    DbPool,
    models::{
        CreateInventory, 
        Inventory, 
    }, 
    error::AppError
}; 
use diesel::{
    insert_into, 
    QueryDsl, 
    RunQueryDsl,
    prelude::*
};

pub async fn create_inventory(
    axum::Extension(pool): axum::Extension<DbPool>,
    Json(payload): Json<CreateInventory<'_>>
) -> Result<Json<Inventory>, axum::Error> {
    let mut conn = pool.get().expect("could not get db connection from pool");
    use crate::database::schema::inventory::dsl::*;

    let new_inventory = CreateInventory {
        lot_number: payload.lot_number,
        location: payload.location,
        quantity: payload.quantity,
        created_by: payload.created_by,
        created_at: chrono::Utc::now().naive_utc()
    };

    let result = insert_into(inventory)
        .values(&new_inventory)
        .execute(&mut conn);  

    match result {
        Ok(_) => {
            let created_inventory: Inventory = inventory
                .order(id.desc())
                .first(&mut conn)
                .expect("Could not retrieve created inventory");
            Ok(Json(created_inventory))
        },
        Err(e) => Err(AppError::from(e).into())  
    }
}

