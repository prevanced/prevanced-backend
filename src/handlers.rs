use crate::{auth::Claims, types::*};
use axum::{http::StatusCode, response::IntoResponse, Json};

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Server is up and running!"
}

pub async fn register(claims: Claims, Json(payload): Json<DeviceRegister>) -> impl IntoResponse {
    let device_id = claims.device_id;
    let fcm_token = payload.fcm_token;

    println!("Storing {} for {}", fcm_token, device_id);

    (StatusCode::CREATED, Json(DeviceRegister { fcm_token }))
}

pub async fn delete_device(
    claims: Claims,
    Json(payload): Json<DeviceRegister>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = payload;

    println!("Deleting {} for {}", user.fcm_token, claims.device_id);

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
