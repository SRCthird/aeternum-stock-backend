use rocket::{fairing::{Fairing, Info, Kind}, http::{uri::Origin, Status}, response::status, Data, Request};
use std::env;
use std::path::Path;
use dotenvy::dotenv;

pub struct ApiKeyFairing;

#[rocket::async_trait]
impl Fairing for ApiKeyFairing {
    fn info(&self) -> Info {
        Info {
            name: "API Key Checker",
            kind: Kind::Request
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        dotenv().ok();
        let api_key = api_key();
        let recieved = request.headers().get_one("x-api-key");
        match recieved {
            Some(key) if key == api_key => {
                // If the API key is correct, do nothing
            },
            _ => {
                let error_uri = Origin::try_from("/api/error/invalid_api_key").expect("valid URI");
                request.set_uri(error_uri);
            }
        }
    }
}

fn api_key() -> String {
    if let Ok(exe_path) = env::current_exe() {
        let dir = exe_path.parent().unwrap_or_else(|| Path::new(""));
        let dotenv_path = dir.join(".env");
        if dotenvy::from_filename(dotenv_path.to_str().unwrap()).is_err() {
            dotenv().ok();
        }
    } else {
        dotenv().ok();
    }

    env::var("API_KEY").expect("API_KEY must be set")
}

#[post("/invalid_api_key")]
pub fn post_invalid_api_key() -> status::Custom<String> {
    status::Custom(Status::Unauthorized, "Invalid API key".to_string())
}
#[get("/invalid_api_key")]
pub fn get_invalid_api_key() -> status::Custom<String> {
    status::Custom(Status::Unauthorized, "Invalid API key".to_string())
}
#[put("/invalid_api_key")]
pub fn put_invalid_api_key() -> status::Custom<String> {
    status::Custom(Status::Unauthorized, "Invalid API key".to_string())
}
#[delete("/invalid_api_key")]
pub fn delete_invalid_api_key() -> status::Custom<String> {
    status::Custom(Status::Unauthorized, "Invalid API key".to_string())
}
#[patch("/invalid_api_key")]
pub fn patch_invalid_api_key() -> status::Custom<String> {
    status::Custom(Status::Unauthorized, "Invalid API key".to_string())
}
#[head("/invalid_api_key")]
pub fn head_invalid_api_key() -> status::Custom<String> {
    status::Custom(Status::Unauthorized, "Invalid API key".to_string())
}
#[options("/invalid_api_key")]
pub fn options_invalid_api_key() -> status::Custom<String> {
    status::Custom(Status::Unauthorized, "Invalid API key".to_string())
}

