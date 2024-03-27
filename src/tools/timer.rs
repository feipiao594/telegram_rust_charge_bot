use mobot::{
    api::{self, Message, SendMessageRequest},
    Client,
};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::{data::chargecny::get_cron_charge_cny, tools::config};

pub async fn timer_init() -> Result<(), JobSchedulerError> {
    let sched = JobScheduler::new().await?;
    let cron: &str = &config::get_instance().event_trigger_time;
    sched
        .add(Job::new_async(cron, |_uuid, _l| {
            Box::pin(async move {
                let token = config::get_instance().bot_token.clone();
                let client = Client::new(token);
                let chargecny_map = get_cron_charge_cny().await;
                let (_, charge_str) = &chargecny_map;
                let req = SendMessageRequest::new(
                    (&config::get_instance().group_chat_id).clone(),
                    charge_str,
                )
                .with_reply_markup(api::ReplyMarkup::reply_keyboard_remove());
                super::charge_store::push_charge_store(chargecny_map)
                    .await
                    .unwrap();
                let _: anyhow::Result<Message> = client.post("sendMessage", &req).await;
            })
        })?)
        .await?;
    sched.start().await?;
    Ok(())
}
