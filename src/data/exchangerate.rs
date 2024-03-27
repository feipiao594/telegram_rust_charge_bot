use std::f64::NAN;

use crate::data::rounded_number;
use crate::tools::{config, http};
use serde;

#[derive(Debug, Clone, serde::Deserialize)]
struct InnerJsonCNY {
    #[serde(rename = "CNY")]
    cny: f64,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ExchangeRate {
    date: String,
    rates: InnerJsonCNY,
}

pub async fn get_exchange_rate_num() -> (f64, String) {
    let url = &config::get_instance().exchange_rate_url;
    // println!("{}", &url);
    let body = http::url_get(&url).await;
    // println!("{}", body);
    match serde_json::from_str::<ExchangeRate>(&body) {
        Ok(parsed_json) => {
            return (rounded_number(parsed_json.rates.cny, 4), parsed_json.date);
        }
        Err(_) => {
            return (NAN, "".to_string());
        }
    }
}

pub async fn get_exchange_rate() -> String {
    format!(
        "当前汇率值: 1 USD = {:.4} CNY",
        get_exchange_rate_num().await.0
    )
}
