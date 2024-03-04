use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

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