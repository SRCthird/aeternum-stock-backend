use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct InventoryBay {
    pub id: i32,
    pub name: String,
    pub warehouse_name: String,
    pub max_unique_lots: i32,
    pub friendly_name: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::inventorybay)]
pub struct CreateInventoryBay {
    pub name: String,
    pub friendly_name: Option<String>,
    pub warehouse_name: String,
    pub max_unique_lots: i32,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::inventorybay)]
pub struct UpdateInventoryBay {
    pub name: Option<String>,
    pub friendly_name: Option<String>,
    pub warehouse_name: Option<String>,
    pub max_unique_lots: Option<i32>,
}
