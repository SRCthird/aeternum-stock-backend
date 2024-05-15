use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::product)]
pub struct CreateProduct {
    pub name: String,
    pub description: String,
}

#[derive(Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = crate::database::schema::product)]
pub struct UpdateProduct {
    pub name: Option<String>,
    pub description: Option<String>,
}

