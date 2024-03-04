use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::{
    net::SocketAddr,
    str::FromStr,
};

pub static API_BASE_URL: Lazy<String> =
    Lazy::new(|| std::env::var("API_BASE_URL").expect("API_BASE_URL is not set"));

pub static API_KEY: Lazy<String> = Lazy::new(|| std::env::var("API_KEY").expect("API_KEY is not set"));

pub static REQWEST_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .resolve("database.deta.sh", SocketAddr::from_str("75.2.69.226:443").expect("Invalid IP"))
        .resolve("database.deta.sh", SocketAddr::from_str("76.223.55.44:443").expect("Invalid IP"))
        .build()
        .expect("Failed to create reqwest client")
});

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
    ReqwestError(reqwest::Error), 
    SerdeError(serde_json::Error),
    Others(String),
}

impl Display for DBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DBError::ReqwestError(e) => write!(f, "ReqwestError: {}", e),
            DBError::SerdeError(e) => write!(f, "SerdeError: {}", e),
            DBError::Others(e) => write!(f, "Others: {}", e),
        }
    }
}