use rocket::{http::{Status, Method}, response::status};
use rocket_cors::{AllowedOrigins, CorsOptions};

pub fn cors() -> CorsOptions { 
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        )
        .allow_credentials(true);
    cors
}

#[options("/cors")]
pub fn good() -> status::Custom<String> {
    status::Custom(Status::Ok, "".to_string())
}
