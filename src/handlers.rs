use crate::{auth::Claims, types::*};
use axum::{http::StatusCode, response::IntoResponse, Json, extract::Query};
use crate::db::{insert, delete, all};

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Server is up and running!"
}

pub async fn register(claims: Claims, Json(payload): Json<DeviceRegister>) -> impl IntoResponse {
    let device_id = claims.device_id;
    let fcm_token = payload.fcm_token;

    insert(&fcm_token, &device_id).await.expect(&format!("Failed to insert {}", fcm_token));

    (StatusCode::CREATED, Json(DeviceRegister { fcm_token }))
}

pub async fn delete_device(
    Json(payload): Json<DeviceRegister>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = payload;

    delete(&user.fcm_token).await.expect(&format!("Failed to delete {}", user.fcm_token));

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(user))
}

#[derive(serde::Deserialize)]
pub struct Last {
    last: Option<String>,
}

pub async fn get_all(Query(params): Query<Last>) -> impl IntoResponse {
    let last = params.last;
    let all = all(last).await.expect("Failed to get all tokens");    

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(all))
}

pub async fn ping() -> &'static str {
    "pong"
}
