use crate::tools::http;
use serde;

#[derive(Debug, Clone, serde::Deserialize)]
struct InnerJsonCNY {
    #[warn(non_snake_case)]
    CNY: f32,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ExchangeRate {
    rates: InnerJsonCNY,
}

pub async fn get_exchange_rate() -> String {
    let url = "https://api.vatcomply.com/rates?base=USD".to_string();
    let body = http::url_get(&url).await.unwrap();
    match serde_json::from_str::<ExchangeRate>(&body) {
        Ok(parsed_json) => {
            return format!("{:?}", parsed_json.rates.CNY);
        }
        Err(e) => {
            return format!("Error parsing JSON: {}", e);
        }
    }
}
