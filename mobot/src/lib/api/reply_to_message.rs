use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct ReplyToMessage {
    /// Unique message identifier inside this reply to message
    pub message_id: i64,
}

impl ReplyToMessage {
    pub fn new(message_id: i64) -> Self {
        Self { message_id }
    }
}
