use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceRegister {
    pub fcm_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushNotification {
    title: String,
    body: String,
}

pub struct DetaSecrets {
    key: String,
}

use once_cell::sync::Lazy;

pub static SECRETS: Lazy<DetaSecrets> = Lazy::new(|| DetaSecrets {
    key: std::env::var("DETA_KEY").expect("DETA_KEY is not set"),
});

