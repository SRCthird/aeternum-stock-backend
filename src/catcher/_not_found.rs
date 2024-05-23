use rocket::{
    Request,
    http::Status, 
    response::status, 
};


#[catch(404)]
pub fn not_found(req: &Request) -> status::Custom<String> {
    status::Custom(Status::NotFound, format!("Sorry, '{}' is not a valid path.", req.uri()))
}
