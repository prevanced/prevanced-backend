use core::fmt;
use std::fmt::{Display, Formatter};

use isahc::error::Error as IsahcError;
use isahc::http::Error as IsahcHttpError;
use isahc::http::{Request, Response};
use isahc::{AsyncReadResponseExt, Body, RequestExt};


mod types;

use types::*;

impl Display for DBError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DBError::IsahcError(e) => write!(f, "IsahcError: {}", e),
            DBError::IsahcHttpError(e) => write!(f, "IsahcHttpError: {}", e),
            DBError::SerdeError(e) => write!(f, "SerdeError: {}", e),
            DBError::Others(e) => write!(f, "Others: {}", e),
        }
    }
}

impl From<IsahcError> for DBError {
    fn from(e: IsahcError) -> Self {
        DBError::IsahcError(e)
    }
}

impl From<IsahcHttpError> for DBError {
    fn from(e: IsahcHttpError) -> Self {
        DBError::IsahcHttpError(e)
    }
}

impl From<serde_json::Error> for DBError {
    fn from(e: serde_json::Error) -> Self {
        DBError::SerdeError(e)
    }
}

impl From<String> for DBError {
    fn from(e: String) -> Self {
        DBError::Others(e)
    }
}

fn send(request: Request<String>) -> Result<Response<Body>, DBError> {
    let response = isahc::send(request)?;

    Ok(response)
}

async fn deta_req(url: String, method: &str, body: Option<String>) -> Result<(), DBError> {
    let headers = [
        ("X-API-Key", API_KEY.as_str()),
        ("Content-Type", "application/json"),
    ];

    let mut request = isahc::Request::builder().method(method).uri(url);

    for item in headers.iter() {
        request = request.header(item.0, item.1);
    }

    let request = request.body(body.unwrap_or_default()).unwrap();

    let response = send(request)?;

    let status = response.status();

    if status.is_success() {
        Ok(())
    } else {
        Err(DBError::from(format!("Status code: {}", status)))
    }
}

pub async fn insert(fcm_token: &str, device_id: &str) -> Result<(), DBError> {
    // POST /items
    let body = serde_json::to_string(&PostItems {
        item: TokenItems {
            key: fcm_token.to_string(),
            device_id: device_id.to_string(),
        },
    })?;

    let uri = format!("{}/{}", API_BASE_URL.as_str(), "items");

    deta_req(uri, "POST", Some(body)).await
}


pub async fn delete(fcm_token: &str) -> Result<(), DBError> {
    // DELETE /items/{key}
    let uri = format!("{}/{}/{}", API_BASE_URL.as_str(), "items", fcm_token);

    deta_req(uri, "DELETE", None).await
}

pub async fn get(fcm_token: &str) -> Result<(), DBError> {
    // GET /items/{key}
    let uri = format!("{}/{}/{}", API_BASE_URL.as_str(), "items", fcm_token);

    deta_req(uri, "GET", None).await
}

pub async fn all(last: String) -> Result<serde_json::Value, DBError> {
    println!("API_BASE_URL: {}", API_BASE_URL.to_string());
    println!("API_KEY: {}", API_KEY.to_string());
    // GET /items
    let uri = format!("{}/{}", API_BASE_URL.as_str(), "query");

    println!("last: {}", last);

    let body = serde_json::to_string(&serde_json::json!({
        "query": [],
        "last": last,
    }))?;

    let mut response = isahc::Request::post(uri)
        .header("X-API-Key", API_KEY.as_str())
        .header("Content-Type", "application/json")
        .body(body)? // TODO: Fix this
        .send_async()
        .await?;

    let status = response.status();

    if status.is_success() {
        let result = response.text().await;
        let body = match result {
            Ok(body) => body,
            Err(e) => return Err(DBError::Others(e.to_string())),
        };

        let items: serde_json::Value = serde_json::from_str(&body)?;

        Ok(items)
    } else {
        Err(DBError::from(format!("Status code: {}", status)))
    }
}
