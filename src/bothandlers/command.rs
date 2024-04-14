use super::ChatState;
use crate::data::*;
use crate::tools::config;
use mobot::{
    api::{SendChatActionRequest, SendMessageRequest},
    *,
};

fn is_in_group(e: &Event) -> bool {
    let id = e.update.chat_id().unwrap();
    log::info!("check request group id: {}", id);
    let mut flag = false;
    if id == config::get_instance().group_chat_id {
        flag = true;
    }
    for &i in &config::get_instance().subscribed_id_list {
        if id == i {
            flag = true;
        }
    }
    flag
}

pub async fn help_command(e: Event, _: State<ChatState>) -> anyhow::Result<Action> {
    log::info!("route message: help, request id: {}", e.update.chat_id()?);
    let chat_id = e.update.chat_id()?;
    let _help_message = &config::get_instance().help_message;
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
                _help_message.to_owned()
                    + "/help 显示该帮助信息 \n\
                        /exchangerate 查询当前汇率 \n\
                        /dataused 查询当前流量 \n\
                        /chargecny 查询最近缴额",
            )
            .with_reply_markup(api::ReplyMarkup::reply_keyboard_remove()),
        )
        .await?;
    log::info!("send message: help, request id: {}", e.update.chat_id()?);
    Ok(Action::Done)
}

pub async fn exchange_rate_command(e: Event, _: State<ChatState>) -> anyhow::Result<Action> {
    log::info!("route message: exchangerate");
    if !is_in_group(&e) {
        log::warn!(
            "request id({}) is not in subscribed list",
            e.update.chat_id()?
        );
        return Ok(Action::ReplyText("抱歉，您未订阅该服务".to_owned()));
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
    log::info!(
        "send message: exchangerate, request id: {}",
        e.update.chat_id()?
    );
    Ok(Action::Done)
}

pub async fn data_used_command(e: Event, _: State<ChatState>) -> anyhow::Result<Action> {
    log::info!("route message: dataused");
    if !is_in_group(&e) {
        log::warn!(
            "request id({}) is not in subscribed list",
            e.update.chat_id()?
        );
        return Ok(Action::ReplyText("抱歉，您未订阅该服务".to_owned()));
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
    log::info!(
        "send message: dataused, request id: {}",
        e.update.chat_id()?
    );
    Ok(Action::Done)
}

pub async fn charge_cny_command(e: Event, _: State<ChatState>) -> anyhow::Result<Action> {
    log::info!("route message: chargecny");
    if !is_in_group(&e) {
        log::warn!(
            "request id({}) is not in subscribed list",
            e.update.chat_id()?
        );
        return Ok(Action::ReplyText("抱歉，您未订阅该服务".to_owned()));
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
    log::info!(
        "send message: chargecny, request id: {}",
        e.update.chat_id()?
    );
    Ok(Action::Done)
}
