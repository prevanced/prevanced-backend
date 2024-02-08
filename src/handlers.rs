use crate::{auth::{AuthError, Claims}, types::*};
use axum::{http::StatusCode, response::IntoResponse, Json};

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Server is up and running!"
}

pub async fn register(claims: Claims, data: Json<DeviceRegister>) -> Result<String, AuthError> {
    let device = RegisteredDevice {
        device_id: claims.device_id,
        fcm_token: data.fcm_token.to_owned(),
    };

    println!("Storing {} for {}", device.fcm_token, device.device_id);

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok(device.fcm_token)
}

pub async fn delete_device(claims: Claims, Json(payload): Json<DeviceRegister>) -> impl IntoResponse {
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
