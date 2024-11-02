use log::info;
use mobot::{
    api::{self, send_local_photo, Message, SendMessageRequest, SendPhotoRequest},
    Client,
};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::{
    data::{chargecny::get_cron_charge_cny, dataused::get_data_used, record_img::get_record_img},
    tools::{
        config,
        history::{append_checks, get_history, get_recent_checks, get_recent_checks_number},
    },
};

pub async fn timer_init() -> Result<(), JobSchedulerError> {
    let sched = JobScheduler::new().await?;
    let cron: &str = &config::get_instance().event_trigger_time;
    sched
        .add(Job::new_async(cron, |_uuid, _l| {
            Box::pin(async move {
                let token = config::get_instance().bot_token.clone();
                let client = Client::new(token);
                let (_, charge_str) = get_cron_charge_cny().await;
                super::history::new_period(&charge_str).await.unwrap();

                let his = get_history().await.unwrap();
                let n = get_recent_checks_number(&his);
                if n <= 5 {
                    let req =
                        SendMessageRequest::new(config::get_instance().group_chat_id, charge_str)
                            .with_reply_markup(api::ReplyMarkup::reply_keyboard_remove());
                    let _: anyhow::Result<Message> = client.post("sendMessage", &req).await;
                } else {
                    let req = SendPhotoRequest::new(
                        config::get_instance().group_chat_id,
                        "image.jpg".to_string(),
                        Some(charge_str),
                    );
                    let _ = send_local_photo(
                        &client,
                        &req,
                        get_record_img(get_recent_checks(&his)).unwrap(),
                    )
                    .await;
                }
            })
        })?)
        .await?;
    let daily_cron = &config::get_instance().daily_cron_time;
    sched
        .add(Job::new_async(daily_cron, |_uuid, _l| {
            Box::pin(async move {
                let data_used = get_data_used().await.unwrap();
                info!("daily check data used: {:?}", data_used);
                append_checks(data_used).await.unwrap();
            })
        })?)
        .await?;
    sched.start().await?;
    log::info!("cron timer initialized");
    Ok(())
}
