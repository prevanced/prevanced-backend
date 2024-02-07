use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize,  Deserialize)]
pub struct DeviceRegister {
    device_id: String,
    fcm_token: String,
}

#[derive(Debug, Serialize,  Deserialize)]
pub struct PushNotification {
    title: String,
    body: String,
}