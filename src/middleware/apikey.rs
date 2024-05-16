use rocket::{fairing::{Fairing, Info, Kind}, http::{uri::Origin, Status}, response::status, Data, Request};
use std::env;
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
        let api_key = env::var("API_KEY").expect("API_KEY must be set");
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

#[get("/invalid_api_key")]
pub fn invalid_api_key() -> status::Custom<String> {
    status::Custom(Status::Unauthorized, "Invalid API key".to_string())
}

