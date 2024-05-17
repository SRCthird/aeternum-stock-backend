use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Inventory {
    pub id: i32,
    pub lot_number: String,
    pub location: String,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub created_by: String,
    pub updated_at: Option<NaiveDateTime>,
    pub updated_by: Option<String>,
    pub from_location: String,
    pub comments: String,
}

#[derive(Debug, Insertable, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::inventory)]
pub struct CreateInventory {
    pub lot_number: String,
    pub location: String,
    pub quantity: i32,
    pub created_by: String,
    pub created_at: Option<NaiveDateTime>,
    pub from_location: String,
    pub comments: String,
}

#[derive(Deserialize, AsChangeset, Debug, Clone)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::inventory)]
pub struct UpdateInventory {
    pub lot_number: Option<String>,
    pub location: Option<String>,
    pub quantity: Option<i32>,
    pub updated_by: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub from_location: Option<String>,
    pub comments: Option<String>,
}
