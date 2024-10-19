use std::borrow::Cow;

use mobot_derive::BotRequest;
use reqwest::multipart;
use serde::Serialize;

use super::{Message, API};

#[derive(Debug, Serialize, Clone, BotRequest)]
pub struct SendPhotoRequest {
    pub chat_id: i64,

    pub photo: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
}

impl SendPhotoRequest {
    pub fn new(chat_id: i64, photo: String, caption: Option<String>) -> Self {
        Self {
            chat_id,
            photo,
            caption,
            message_thread_id: None,
        }
    }
}

/// API methods for sending photo.
impl API {
    /// Send a phoyo.
    pub async fn send_url_photo(&self, req: &SendPhotoRequest) -> anyhow::Result<Message> {
        self.client.post("sendPhoto", req).await
    }

    pub async fn send_local_photo<T>(
        &self,
        req: &SendPhotoRequest,
        photo_data: T,
    ) -> anyhow::Result<Message>
    where
        T: Into<Cow<'static, [u8]>>,
    {
        let mut form = multipart::Form::new()
            .text("chat_id", req.chat_id.clone().to_string())
            .part(
                "photo",
                multipart::Part::bytes(photo_data)
                    .file_name(req.photo.clone())
                    .mime_str("image/jpeg")?,
            );

        if let Some(caption) = &req.caption {
            form = form.text("caption", caption.clone().to_string());
        }

        if let Some(message_thread_id) = &req.message_thread_id {
            form = form.text("message_thread_id", message_thread_id.clone().to_string());
        }

        self.client.multipart_post("sendPhoto", form).await
    }
}
