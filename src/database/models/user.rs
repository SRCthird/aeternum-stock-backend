use diesel::prelude::*;
use rocket::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

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
    pub role: Option<String>,
    pub position: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub created_at: Option<NaiveDateTime>,
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
    pub updated_at: Option<NaiveDateTime>,
}
