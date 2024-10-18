use mobot_derive::BotRequest;
use serde::Serialize;

use super::API;

#[derive(Debug, Serialize, Clone, BotRequest)]
pub struct SendPhotoRequest {
    pub chat_id: i64,

    pub photo: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
}

impl SendPhotoRequest {
    pub fn new(chat_id: i64, photo: String) -> Self {
        Self {
            chat_id,
            photo,
            message_thread_id: None,
        }
    }
}

/// API methods for sending photo.
impl API {
    /// Send a message.
    pub async fn send_photo(&self, req: &SendPhotoRequest) -> anyhow::Result<bool> {
        self.client.post("sendPhoto", req).await
    }
}
