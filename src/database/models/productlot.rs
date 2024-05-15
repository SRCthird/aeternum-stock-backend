use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct ProductLot {
    pub id: i32,
    pub lot_number: String,
    pub internal_reference: String,
    pub product_name: String,
    pub quantity: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::productlot)]
pub struct CreateProductLot {
    pub lot_number: String,
    pub internal_reference: String,
    pub product_name: String,
    pub quantity: i32,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::productlot)]
pub struct UpdateProductLot {
    pub lot_number: Option<String>,
    pub internal_reference: Option<String>,
    pub product_name: Option<String>,
    pub quantity: Option<i32>,
}
