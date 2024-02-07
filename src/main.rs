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
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/register", post(register_device));

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
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<DeviceRegister>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = payload;

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the output to our `create_user` handler
#[derive(Debug, Serialize,  Deserialize)]
struct DeviceRegister {
    device_id: String,
    fcm_token: String,
}
