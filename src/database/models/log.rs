use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Log {
    pub id: i32,
    pub from_location: String,
    pub to_location: String,
    pub date_time: NaiveDateTime,
    pub user: String,
    pub lot_number: String,
    pub quantity_moved: i32,
    pub comments: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::log)]
pub struct CreateLog {
    pub from_location: String,
    pub to_location: String,
    pub user: String,
    pub lot_number: String,
    pub quantity_moved: i32,
    pub comments: String,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::log)]
pub struct UpdateLog {
    pub from_location: Option<String>,
    pub to_location: Option<String>,
    pub user: Option<String>,
    pub lot_number: Option<String>,
    pub quantity_moved: Option<i32>,
    pub comments: Option<String>,
}
