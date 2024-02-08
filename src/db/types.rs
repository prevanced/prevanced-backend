use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use isahc::error::Error as IsahcError;
use isahc::http::Error as IsahcHttpError;

pub static API_BASE_URL: Lazy<String> =
    Lazy::new(|| std::env::var("API_BASE_URL").expect("API_BASE_URL is not set"));

pub static API_KEY: Lazy<String> = Lazy::new(|| std::env::var("API_KEY").expect("API_KEY is not set"));

#[derive(Serialize, Deserialize)]
pub struct TokenItems {
    pub key: String,
    pub device_id: String,
}

#[derive(Serialize)]
pub struct PostItems {
    pub items: Vec<TokenItems>,
}

#[derive(Debug)]
pub enum DBError {
    IsahcError(IsahcError),
    IsahcHttpError(IsahcHttpError),
    SerdeError(serde_json::Error),
    Others(String),
}