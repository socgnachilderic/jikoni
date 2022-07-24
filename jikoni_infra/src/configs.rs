use std::{borrow::Cow, env};

use actix_cors::Cors;
use actix_web::http::header;

#[derive(Debug, Clone)]
pub struct Env {
    pub db_url: Cow<'static, str>,
    pub host: Cow<'static, str>,
    pub port: u16,
    pub media_root: Cow<'static, str>,
}

impl Env {
    pub fn initialize_environnement() -> Self {
        dotenv::dotenv().ok();

        let db_url =
            env::var("DATABASE_URL").expect(&Self::format_expected_message("DATABASE_URL"));

        let media_root =
            env::var("JIKONI_APP_MEDIA_ROOT").expect(&Self::format_expected_message("JIKONI_APP_MEDIA_ROOT"));
        let host = env::var("JIKONI_APP_HOST").expect(&Self::format_expected_message("JIKONI_APP_HOST"));
        let port = env::var("JIKONI_APP_PORT").expect(&Self::format_expected_message("JIKONI_APP_PORT"));
        let port = port.parse::<u16>().expect("PORT is not valid number");

        Self {
            db_url: Cow::from(db_url),
            host: Cow::from(host),
            media_root: Cow::from(media_root),
            port,
        }
    }

    pub fn get_server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    fn format_expected_message(obj: &str) -> String {
        format!("{obj} is not set in .env file")
    }
}

pub fn get_cors() -> Cors {
    Cors::default()
        .allow_any_origin()
        .allowed_methods(["POST", "GET"])
        .allowed_headers([header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .supports_credentials()
        .max_age(3600)
}
