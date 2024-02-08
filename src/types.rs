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

#[derive(Debug, Deserialize)]
pub struct RegisteredDevice {
    pub device_id: String,
    pub fcm_token: String,
}