use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceRegister {
    pub fcm_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushNotification {
    title: String,
    body: String,
}
