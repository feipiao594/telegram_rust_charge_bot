use super::ChatState;
use crate::data::*;
use mobot::{api::SendMessageRequest, *};

pub async fn help_command(e: Event, _: State<ChatState>) -> Result<Action, anyhow::Error> {
    let chat_id = e.update.chat_id()?;
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

pub async fn exchange_rate_command(e: Event, _: State<ChatState>) -> Result<Action, anyhow::Error> {
    let chat_id = e.update.chat_id()?;
    let message = exchangerate::get_exchange_rate().await;
    e.api
        .send_message(
            &SendMessageRequest::new(chat_id, message)
                .with_reply_markup(api::ReplyMarkup::reply_keyboard_remove()),
        )
        .await?;

    Ok(Action::Done)
}

pub async fn data_used_command(e: Event, _: State<ChatState>) -> Result<Action, anyhow::Error> {
    let chat_id = e.update.chat_id()?;
    let message = dataused::get_used_data().await;
    e.api
        .send_message(
            &SendMessageRequest::new(chat_id, message)
                .with_reply_markup(api::ReplyMarkup::reply_keyboard_remove()),
        )
        .await?;

    Ok(Action::Done)
}

pub async fn charge_cny_command(e: Event, _: State<ChatState>) -> Result<Action, anyhow::Error> {
    let chat_id = e.update.chat_id()?;
    let message = chargecny::get_cron_charge_cny().await;
    e.api
        .send_message(
            &SendMessageRequest::new(chat_id, message)
                .with_reply_markup(api::ReplyMarkup::reply_keyboard_remove()),
        )
        .await?;

    Ok(Action::Done)
}
