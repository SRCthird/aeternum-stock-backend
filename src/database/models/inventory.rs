use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Queryable)]
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
}


#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::inventory)]
pub struct CreateInventory<'a> {
    pub lot_number: &'a str,
    pub location: &'a str,
    pub quantity: i32,
    pub created_by: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::inventory)]
pub struct UpdateInventory {
    pub lot_number: Option<String>,
    pub location: Option<String>,
    pub quantity: Option<i32>,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}
