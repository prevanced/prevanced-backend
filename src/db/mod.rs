use reqwest::{header::HeaderMap, Error, Method, RequestBuilder, Response};

mod types;

use types::*;

impl From<Error> for DBError {
    fn from(e: Error) -> Self {
        DBError::ReqwestError(e)
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

async fn send(request: RequestBuilder) -> Result<Response, DBError> {
    let request_body = request.build()?;
    let response = REQWEST_CLIENT.execute(request_body).await?;
    Ok(response)
}

async fn deta_req(url: String, method: &str, body: Option<String>) -> Result<(), DBError> {
    let mut headers = HeaderMap::new();
    headers.insert("X-API-Key", API_KEY.parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let request = REQWEST_CLIENT
        .request(Method::from_bytes(method.as_bytes()).unwrap(), url)
        .headers(headers);

    let request = match body {
        Some(body_str) => request.body(body_str),
        None => request,
    };

    let response = send(request).await?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(DBError::from(format!("Status code: {}", response.status())))
    }
}

pub async fn insert(fcm_token: &str, device_id: &str) -> Result<(), DBError> {
    // POST /items
    let body = serde_json::to_string(&PostItems {
        items: vec![TokenItems {
            key: fcm_token.to_string(),
            device_id: device_id.to_string(),
        }],
    })?;
    let uri = format!("{}/{}", API_BASE_URL.as_str(), "items");

    deta_req(uri, "PUT", Some(body)).await
}

pub async fn delete(fcm_token: &str) -> Result<(), DBError> {
    // DELETE /items/{key}
    let uri = format!("{}/{}/{}", API_BASE_URL.as_str(), "items", fcm_token);

    deta_req(uri, "DELETE", None).await
}

// pub async fn get(fcm_token: &str) -> Result<(), DBError> {
//     // GET /items/{key}
//     let uri = format!("{}/{}/{}", API_BASE_URL.as_str(), "items", fcm_token);

//     deta_req(uri, "GET", None).await
// }

pub async fn all(last: Option<String>) -> Result<serde_json::Value, DBError> {
    // GET /items
    let uri = format!("{}/{}", API_BASE_URL.as_str(), "query");

    let body = serde_json::to_string(&serde_json::json!({
        "query": [],
        "last": last,
    }))?;

    let response = REQWEST_CLIENT
        .post(uri)
        .header("X-API-Key", API_KEY.as_str())
        .header("Content-Type", "application/json")
        .body(body)
        .send()
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
