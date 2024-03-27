use super::ChatState;
use crate::data::*;
use crate::tools::config;
use mobot::{
    api::{SendChatActionRequest, SendMessageRequest},
    *,
};

fn is_in_group(e: &Event) -> bool {
    let id = e.update.chat_id().unwrap();
    if id == config::get_instance().group_chat_id {
        return true;
    }
    for &i in &config::get_instance().subscribed_id_list {
        if id == i {
            return true;
        }
    }
    false
}

pub async fn help_command(e: Event, _: State<ChatState>) -> anyhow::Result<Action> {
    // println!("help_command");
    let chat_id = e.update.chat_id()?;
    e.api
        .send_chat_action(&SendChatActionRequest::new(
            chat_id,
            api::ChatAction::Typing,
        ))
        .await?;
    // println!("send_message");
    e.api
        .send_message(
            &SendMessageRequest::new(
                chat_id,
                "你好，欢迎使用此 bot\n\
                        该 bot 用于查询 JMS 相关的信息\n\
                        北京时间 20 号 12 点自动查询本月应缴额\n\
                        /help 显示该帮助信息 \n\
                        /exchangerate 查询当前汇率 \n\
                        /dataused 查询当前流量 \n\
                        /chargecny 查询最近缴额",
            )
            .with_reply_markup(api::ReplyMarkup::reply_keyboard_remove()),
        )
        .await?;

    Ok(Action::Done)
}

pub async fn exchange_rate_command(e: Event, _: State<ChatState>) -> anyhow::Result<Action> {
    if !is_in_group(&e) {
        return Ok(Action::ReplyText(format!("抱歉，您未订阅该服务")));
    }
    let chat_id = e.update.chat_id()?;
    e.api
        .send_chat_action(&SendChatActionRequest::new(
            chat_id,
            api::ChatAction::Typing,
        ))
        .await?;
    // println!("send_message");
    let message = exchangerate::get_exchange_rate().await;
    e.api
        .send_message(
            &SendMessageRequest::new(chat_id, message)
                .with_reply_markup(api::ReplyMarkup::reply_keyboard_remove()),
        )
        .await?;

    Ok(Action::Done)
}

pub async fn data_used_command(e: Event, _: State<ChatState>) -> anyhow::Result<Action> {
    if !is_in_group(&e) {
        return Ok(Action::ReplyText(format!("抱歉，您未订阅该服务")));
    }
    let chat_id = e.update.chat_id()?;
    e.api
        .send_chat_action(&SendChatActionRequest::new(
            chat_id,
            api::ChatAction::Typing,
        ))
        .await?;
    // println!("send_message");
    let message = dataused::get_used_data().await;
    e.api
        .send_message(
            &SendMessageRequest::new(chat_id, message)
                .with_reply_markup(api::ReplyMarkup::reply_keyboard_remove()),
        )
        .await?;

    Ok(Action::Done)
}

pub async fn charge_cny_command(e: Event, _: State<ChatState>) -> anyhow::Result<Action> {
    if !is_in_group(&e) {
        return Ok(Action::ReplyText(format!("抱歉，您未订阅该服务")));
    }
    let chat_id = e.update.chat_id()?;
    e.api
        .send_chat_action(&SendChatActionRequest::new(
            chat_id,
            api::ChatAction::Typing,
        ))
        .await?;
    // println!("send_message");
    let message = chargecny::get_recent_charge_cny().await;
    e.api
        .send_message(
            &SendMessageRequest::new(chat_id, message)
                .with_reply_markup(api::ReplyMarkup::reply_keyboard_remove()),
        )
        .await?;

    Ok(Action::Done)
}
