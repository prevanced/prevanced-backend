use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceRegister {
    pub fcm_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushNotification {
    pub title: String,
    pub body: String,
}
