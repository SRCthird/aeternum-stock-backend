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

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::inventory)]
pub struct CreateInventory<'a> {
    pub lot_number: &'a str,
    pub location: &'a str,
    pub quantity: i32,
    pub created_by: &'a str,
    pub created_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::database::schema::inventory)]
pub struct UpdateInventory {
    pub lot_number: Option<String>,
    pub location: Option<String>,
    pub quantity: Option<i32>,
    pub updated_by: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable)]
pub struct InventoryBay {
    pub id: i32,
    pub name: String,
    pub warehouse_name: String,
    pub max_unique_lots: i32,
}

#[derive(Queryable)]
pub struct CreateInventoryBay {
    pub name: String,
    pub warehouse_name: String,
    pub max_unique_lots: i32,
}

#[derive(Queryable)]
pub struct UpdateInventoryBay {
    pub name: Option<String>,
    pub warehouse_name: Option<String>,
    pub max_unique_lots: Option<i32>,
}

#[derive(Queryable)]
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

#[derive(Queryable)]
pub struct CreateLog {
    pub from_location: String,
    pub to_location: String,
    pub user: String,
    pub lot_number: String,
    pub quantity_moved: i32,
    pub comments: String,
}

#[derive(Queryable)]
pub struct UpdateLog {
    pub from_location: Option<String>,
    pub to_location: Option<String>,
    pub user: Option<String>,
    pub lot_number: Option<String>,
    pub quantity_moved: Option<i32>,
    pub comments: Option<String>,
}

#[derive(Queryable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Queryable)]
pub struct CreateProduct {
    pub name: String,
    pub description: String,
}

#[derive(Queryable)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Queryable)]
pub struct ProductLot {
    pub id: i32,
    pub lot_number: String,
    pub internal_reference: String,
    pub product_name: String,
    pub quantity: i32,
}

#[derive(Queryable)]
pub struct CreateProductLot {
    pub lot_number: String,
    pub internal_reference: String,
    pub product_name: String,
    pub quantity: i32,
}

#[derive(Queryable)]
pub struct UpdateProductLot {
    pub lot_number: Option<String>,
    pub internal_reference: Option<String>,
    pub product_name: Option<String>,
    pub quantity: Option<i32>,
}

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub role: String,
    pub position: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::user)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
    pub role: String,
    pub position: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::user)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<String>,
    pub position: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
}

#[derive(Queryable)]
pub struct Warehouse {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct CreateOrUpdateWarehouse {
    pub name: String,
}
