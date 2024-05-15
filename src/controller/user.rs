use diesel::prelude::*;
use rocket::{
    serde::json::Json,
    http::Status,
    response::status,
};
use crate::database::{
    self,
    models::user::{User, CreateUser, UpdateUser},
    schema::user::dsl,
};

#[post("/", data = "<_user>")]
pub fn input(_user: Json<CreateUser>) -> Result<Json<User>, status::Custom<String>> {
    use crate::database::schema::user;
    let connection = &mut database::establish_connection();
    
    let result = diesel::insert_into(user::table)
        .values(_user.into_inner())
        .execute(connection);

    match result {
        Ok(_) => {
            let inserted_user = dsl::user
                .order(dsl::id.desc())
                .first::<User>(connection)
                .expect("Error getting inserted user");
            Ok(Json(inserted_user))
        },
        Err(e) => Err(
            match e {
                diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => status::Custom(Status::Conflict, "User already exists".to_string()),
                _ => status::Custom(Status::InternalServerError, "Error inserting user".to_string()),
            }
        ),
    }
}

#[get("/?<email>")]
pub fn get(email: Option<String>) -> Result<Json<Vec<User>>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let query_result: QueryResult<Vec<User>> = match email {
        Some(email) => {
            dsl::user.filter(dsl::email.like(format!("{}%", email))).load(connection)
        }
        None => dsl::user.load(connection)
    };

    match query_result {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(status::Custom(Status::InternalServerError, "Error getting users".to_string())),
    }
}

#[get("/<id>")]
pub fn get_one(id: i32) -> Result<Json<User>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = dsl::user
         .filter(dsl::id.eq(id))
         .first::<User>(connection);

    match result {
        Ok(found_user) => Ok(Json(found_user)),
        Err(_) => Err(status::Custom(Status::NotFound, "User not found".to_string())),
    }
}

#[patch("/<id>", data = "<patch_user>")]
pub fn update(id: i32, patch_user: Json<UpdateUser>) -> Result<Json<User>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    
    let update_result = diesel::update(dsl::user.filter(dsl::id.eq(id)))
        .set(&patch_user.into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => {
            match dsl::user.filter(dsl::id.eq(id)).first::<User>(connection) {
                Ok(updated_user) => Ok(Json(updated_user)),
                Err(_) => Err(status::Custom(Status::NotFound, "User not found after update".to_string())),
            }
        },
        Err(_) => Err(status::Custom(Status::InternalServerError, "Error updating user".to_string())),
    }
}

#[delete("/<id>")]
pub fn delete(id: i32) -> Result<Json<User>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = dsl::user
         .filter(dsl::id.eq(id))
         .first::<User>(connection);

    match result {
        Ok(found_user) => {
            diesel::delete(dsl::user.filter(dsl::id.eq(id)))
                .execute(connection).expect("Error deleting sighting");
            Ok(Json(found_user))
        },
        Err(e) => Err(
            match e {
                diesel::result::Error::NotFound => status::Custom(Status::NotFound, "User not found".to_string()),
                _ => status::Custom(Status::InternalServerError, "Error deleting user".to_string()),
            }
        )
    }
}
