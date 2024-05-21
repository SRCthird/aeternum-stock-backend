use crate::database::{
    self,
    models::user::{CreateUser, UpdateUser, User},
    schema::user::{dsl, table},
};
use diesel::{prelude::*, result::{DatabaseErrorKind::UniqueViolation, Error::{DatabaseError, NotFound}}};
use rocket::{http::Status, response::status, serde::json::Json};

#[post("/", data = "<user>")]
pub fn input(user: Json<CreateUser>) -> Result<Json<User>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    let result = diesel::insert_into(table)
        .values(user.into_inner())
        .execute(connection);

    match result {
        Ok(_) => {
            let inserted_user = table.order(dsl::id.desc()).first(connection).unwrap();
            Ok(Json(inserted_user))
        }
        Err(e) => Err(match e {
            NotFound => status::Custom(Status::NotFound, "User not found".to_string()),
            _ => status::Custom(
                Status::InternalServerError,
                "Error inserting user".to_string(),
            ),
        }),
    }
}

#[get("/?<email>")]
pub fn get(email: Option<String>) -> Result<Json<Vec<User>>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    match &email {
        Some(email) => {
            if email.contains(";") || email.contains("%") {
                return Err(status::Custom(
                    Status::BadRequest,
                    "Invalid character in search query".to_string(),
                ));
            }
        }
        None => (),
    }

    let query_result: QueryResult<Vec<User>> = match email {
        Some(email) => dsl::user
            .filter(dsl::email.like(format!("{}%", email)))
            .load(connection),
        None => dsl::user.load(connection),
    };

    match query_result {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(status::Custom(
            Status::InternalServerError,
            "Error getting users".to_string(),
        )),
    }
}

#[get("/<id>")]
pub fn get_one(id: i32) -> Result<Json<User>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let result = dsl::user.filter(dsl::id.eq(id)).first::<User>(connection);

    match result {
        Ok(found_user) => Ok(Json(found_user)),
        Err(_) => Err(status::Custom(
            Status::NotFound,
            "User not found".to_string(),
        )),
    }
}

#[patch("/<id>", data = "<patch_user>")]
pub fn update(id: i32, patch_user: Json<UpdateUser>) -> Result<Json<User>, status::Custom<String>> {
    let connection = &mut database::establish_connection();
    let found_user = dsl::user
        .filter(dsl::id.eq(id))
        .first::<User>(connection)
        .map_err(|e| match e {
            NotFound => status::Custom(Status::NotFound, "User not found in system".to_string()),
            _ => status::Custom(
                Status::InternalServerError,
                "Error loading user".to_string(),
            ),
        })?;

    let update_result = diesel::update(dsl::user.filter(dsl::id.eq(id)))
        .set(&patch_user.into_inner())
        .execute(connection);

    match update_result {
        Ok(_) => Ok(Json(found_user)),
        Err(e) => Err(match e {
            DatabaseError(UniqueViolation, _) => {
                status::Custom(Status::Conflict, "Email already in use".to_string())
            },
            _ => status::Custom(
                Status::InternalServerError,
                "Error updating user".to_string(),
            ),
        }),
    }
}

#[delete("/<id>")]
pub fn delete(id: i32) -> Result<Json<User>, status::Custom<String>> {
    let connection = &mut database::establish_connection();

    let found_user = dsl::user
        .filter(dsl::id.eq(id))
        .first::<User>(connection)
        .map_err(|e| match e {
            NotFound => status::Custom(Status::NotFound, "User not found in system".to_string()),
            _ => status::Custom(
                Status::InternalServerError,
                "Error loading user".to_string(),
            ),
        })?;

    let result = diesel::delete(dsl::user.filter(dsl::id.eq(id))).execute(connection);

    match result {
        Ok(_) => Ok(Json(found_user)),
        Err(_) => Err(status::Custom(
            Status::NotFound,
            "User not found".to_string(),
        )),
    }
}
