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
        if request.method() == rocket::http::Method::Options {
            let uri = Origin::try_from("/api/error/cors").expect("valid URI");
            request.set_uri(uri);
            return;
        }
        dotenv().ok();
        let api_key = api_key();
        let recieved = request.headers().get_one("x-api-key");
        match recieved {
            Some(key) if key == api_key => { },
            _ => {
                let error_uri = Origin::try_from("/api/error/invalid_api_key").expect("valid URI");
                request.set_method(rocket::http::Method::Get);
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

#[get("/invalid_api_key")]
pub fn invalid() -> status::Custom<String> {
    status::Custom(Status::Unauthorized, "Invalid API key".to_string())
}
