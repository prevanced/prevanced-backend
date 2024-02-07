use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/ping", get(ping))
        .route("/register", post(register_device))
        .route("/delete", post(delete_device))
        .route("/send", post(send_notification));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let address: String = std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", address, port))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Server is up and running!"
}

async fn register_device(
    Json(payload): Json<DeviceRegister>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = payload;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

#[derive(Debug, Serialize,  Deserialize)]
struct DeviceRegister {
    device_id: String,
    fcm_token: String,
}

async fn delete_device(
    Json(payload): Json<DeviceRegister>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = payload;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(user))
}

#[derive(Debug, Serialize,  Deserialize)]
struct PushNotification {
    title: String,
    body: String,
}

async fn send_notification(
    Json(payload): Json<PushNotification>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = payload;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::OK, Json(user))
}

async fn ping() -> &'static str {
    "pong"
}
