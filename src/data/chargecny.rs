use crate::data::exchangerate::get_exchange_rate_num;
use crate::data::rounded_number;
use crate::tools::charge_store::get_recent_store;
use crate::tools::config;

pub async fn get_recent_charge_cny() -> String {
    get_recent_store().await.unwrap()
}

pub async fn get_cron_charge_cny() -> (String, String) {
    let (exchangerate, date) = get_exchange_rate_num().await;
    let subscribed_money = config::get_instance().subscribed_money;
    let expand_rate = config::get_instance().expand_rate;
    let subscribed_count = config::get_instance().subscribed_count;
    let str = format!(
        "本月所取汇率值为: 1 USD = {0:.4} CNY\n\
         本月需付: {1} * {0:.4} * {2} / {3} = {4:.2} CNY",
        exchangerate,
        subscribed_money,
        expand_rate,
        subscribed_count,
        rounded_number(
            subscribed_money * expand_rate * exchangerate / (subscribed_count as f64),
            2
        )
    );
    (date, str)
}
