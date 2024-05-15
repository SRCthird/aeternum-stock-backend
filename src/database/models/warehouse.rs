use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Warehouse {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::warehouse)]
pub struct CreateWarehouse {
    pub name: String,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::warehouse)]
pub struct UpdateWarehouse {
    pub name: String,
}
