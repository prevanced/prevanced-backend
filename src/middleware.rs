use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use once_cell::sync::Lazy;

struct SECRETS {
    notification_secret: String,
}

static NOTIFICATION_SECRET: Lazy<SECRETS> = Lazy::new(|| SECRETS {
    notification_secret: std::env::var("NOTIFICATION_SECRET").expect("NOTIFICATION_SECRET is not set"),
});

// Define your custom middleware for key-based authentication
pub async fn key_auth(
    // run the `HeaderMap` extractor
    headers: HeaderMap,
    // you can also add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match get_token(&headers) {
        Some(token) if token_is_valid(token) => {
            let response = next.run(request).await;
            Ok(response)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

fn get_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
}

fn token_is_valid(token: &str) -> bool {
    token == NOTIFICATION_SECRET.notification_secret
}
