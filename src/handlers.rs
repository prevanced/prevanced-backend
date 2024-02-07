use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::types::*;

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Server is up and running!"
}

pub async fn register_device(Json(payload): Json<DeviceRegister>) -> impl IntoResponse {
    // insert your application logic here
    let user = payload;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
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
