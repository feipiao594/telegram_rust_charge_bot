use mobot::{
    api::{self, Message, SendMessageRequest},
    Client,
};
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::{data::chargecny::get_month_charge_cny, tools::config};

pub async fn timer_init() -> Result<(), JobSchedulerError> {
    let sched = JobScheduler::new().await?;
    let cron: &str = &config::get_instance().event_trigger_time;
    sched
        .add(Job::new_async(cron, |_uuid, _l| {
            Box::pin(async move {
                let token = config::get_instance().bot_token.clone();
                println!("I run every {}", &config::get_instance().event_trigger_time);
                let client = Client::new(token);
                let req = SendMessageRequest::new(6151998819, get_month_charge_cny().await)
                    .with_reply_markup(api::ReplyMarkup::reply_keyboard_remove());
                let _: anyhow::Result<Message> = client.post("sendMessage", &req).await;
            })
        })?)
        .await?;
    sched.start().await?;
    Ok(())
}
