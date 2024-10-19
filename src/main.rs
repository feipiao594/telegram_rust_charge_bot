use bothandlers::*;
use charts_rs::get_or_try_init_fonts;
use mobot::*;
use tokio::fs;
use tools::{config, timer};
mod bothandlers;
mod data;
mod tools;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let buf = fs::read(config::get_instance().font_path.clone())
        .await
        .unwrap();
    let _ = get_or_try_init_fonts(Some(vec![&buf]));
    log::info!("read fonts finish");
    let token = config::get_instance().bot_token.clone();
    let client = Client::new(token);
    log::info!("starting bot");
    timer::timer_init().await.unwrap();
    // let mut router = Router::<()>::new(client);

    // router.add_route(Route::Default, |_, _| async move {
    //     Ok(Action::ReplyText("Hello world!".into()))
    // });
    // router.start().await;
    Router::new(client)
        // .add_route(Route::Default, handlers::log_handler)
        .add_route(
            Route::Message(Matcher::BotCommand("help".to_string())),
            command::help_command,
        )
        .add_route(
            Route::Message(Matcher::BotCommand("dataused".to_string())),
            command::data_used_command,
        )
        .add_route(
            Route::Message(Matcher::BotCommand("chargecny".to_string())),
            command::charge_cny_command,
        )
        .add_route(
            Route::Message(Matcher::BotCommand("exchangerate".to_string())),
            command::exchange_rate_command,
        )
        .add_route(
            Route::ChannelPost(Matcher::BotCommand("help".to_string())),
            command::help_command,
        )
        .add_route(
            Route::ChannelPost(Matcher::BotCommand("dataused".to_string())),
            command::data_used_command,
        )
        .add_route(
            Route::ChannelPost(Matcher::BotCommand("chargecny".to_string())),
            command::charge_cny_command,
        )
        .add_route(
            Route::ChannelPost(Matcher::BotCommand("exchangerate".to_string())),
            command::exchange_rate_command,
        )
        .start()
        .await;
}
