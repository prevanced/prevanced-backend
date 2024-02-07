use axum::{
    routing::{get, post},
    Router,
};
mod handlers;
mod types;

use handlers::*;

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
