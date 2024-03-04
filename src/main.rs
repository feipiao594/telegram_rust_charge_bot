mod command;
mod data;
mod tools;
use command::Command;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    // let k = data::dataused::get_used_data().await;
    // println!("{}", k);
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    command::Command::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::ExchangeRate => {
            let rate = data::exchangerate::get_exchange_rate().await;
            let rounded_number =
                |original_number: f64| (original_number * 10000.0).floor() / 10000.0;
            bot.send_message(
                msg.chat.id,
                format!(
                    "当前汇率值: 1 USD = {:.4} CNY",
                    rounded_number(rate.parse::<f64>().unwrap())
                ),
            )
            .await?
        }
        Command::DataUsed => {
            bot.send_message(msg.chat.id, data::dataused::get_used_data().await)
                .await?
        }
        Command::ChargeCNY => bot.send_message(msg.chat.id, format!("TO DO")).await?,
    };

    Ok(())
}
