use diesel::prelude::*;
use rocket::{
    serde::json::Json,
    http::Status,
    response::status,
};

use crate::database::{
    self,
    models::user::{User, CreateUser, UpdateUser},
    schema::user::dsl::*,
};

#[post("/", data = "<_user>")]
pub fn input(_user: Json<CreateUser>) -> Json<User> {
    use crate::database::schema::user;

    let connection = &mut database::establish_connection();
    diesel::insert_into(user::table)
        .values(_user.into_inner())
        .execute(connection)
        .expect("Error adding sighting");

    Json(user::table
        .order(user::id.desc())
        .first(connection).unwrap()
    )
}

#[get("/")]
pub fn get() -> Json<Vec<User>> {
    let connection = &mut database::establish_connection();
    user.load::<User>(connection)
        .map(Json)
        .expect("Error loading birds")
}

#[get("/<_id>")]
pub fn get_one(_id: i32) -> Result<Json<User>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = user
         .filter(id.eq(_id))
         .first::<User>(connection);

    match result {
        Ok(found_user) => Ok(Json(found_user)),
        Err(_) => Err(status::Custom(Status::NotFound, "User not found".to_string())),
    }
}

#[patch("/<_id>", data = "<patch_user>")]
pub fn update(_id: i32, patch_user: Json<UpdateUser>) -> Result<Json<User>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    
    let update_result = diesel::update(user.filter(id.eq(_id)))
        .set(&patch_user.into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => {
            match user.filter(id.eq(_id)).first::<User>(connection) {
                Ok(updated_user) => Ok(Json(updated_user)),
                Err(_) => Err(status::Custom(Status::NotFound, "User not found after update".to_string())),
            }
        },
        Err(_) => Err(status::Custom(Status::InternalServerError, "Error updating user".to_string())),
    }
}

#[delete("/<_id>")]
pub fn delete(_id: i32) -> Result<Json<User>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = user
         .filter(id.eq(_id))
         .first::<User>(connection);

    match result {
        Ok(found_user) => {
            diesel::delete(user.filter(id.eq(_id)))
                .execute(connection).expect("Error deleting sighting");
            Ok(Json(found_user))
        },
        Err(_) => Err(status::Custom(Status::NotFound, "User not found".to_string())),
    }
}
