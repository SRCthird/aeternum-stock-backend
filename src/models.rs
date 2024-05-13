use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable)]
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

#[derive(Queryable)]
pub struct InventoryBay {
    pub id: i32,
    pub name: String,
    pub warehouse_name: String,
    pub max_unique_lots: i32,
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
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
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

#[derive(Queryable)]
pub struct Warehouse {
    pub id: i32,
    pub name: String,
}
