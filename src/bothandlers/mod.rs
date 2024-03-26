pub mod command;

use mobot::BotState;

#[derive(Clone, Default, BotState)]
pub struct ChatState {}

// pub async fn handle_chat_callback(e: Event, _: State<ChatState>) -> Result<Action, anyhow::Error> {
//     let response = format!("Okay: {}", e.update.data().unwrap_or("no callback data"));

//     e.api
//         .send_message(
//             &SendMessageRequest::new(e.update.chat_id()?, response)
//                 .with_reply_markup(api::ReplyMarkup::reply_keyboard_remove()),
//         )
//         .await?;

//     e.remove_inline_keyboard().await?;
//     Ok(Action::Done)
// }
