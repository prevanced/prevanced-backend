use crate::types::*;
use axum::{http::StatusCode, response::IntoResponse, Json};

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Server is up and running!"
}

pub async fn delete_device(Json(payload): Json<DeviceRegister>) -> impl IntoResponse {
    // insert your application logic here
    let user = payload;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(user))
}

pub async fn send_notification(Json(payload): Json<PushNotification>) -> impl IntoResponse {
    // insert your application logic here
    let user = payload;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(user))
}

pub async fn ping() -> &'static str {
    "pong"
}
