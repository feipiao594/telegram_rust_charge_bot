use crate::tools::http;
use serde;

#[derive(Debug, Clone, serde::Deserialize)]
struct DataUsed {
    monthly_bw_limit_b: f64,
    bw_counter_b: f64,
}

pub async fn get_used_data() -> String {
    let url = "https://justmysocks.net/members/getbwcounter.php?service=785853&id=a233a171-2ac4-4213-8ca7-182de981592e".to_string();
    let body = http::url_get(&url).await.unwrap();
    let rounded_number = |original_number: f64| (original_number * 100.0).floor() / 100.0;
    match serde_json::from_str::<DataUsed>(&body) {
        Ok(parsed_json) => {
            let result = format!(
                "每月总额: {:.2} GiB = {:.2} GB\n已用流量: {:.2} GiB = {:.2} GB\n剩余流量: {:.2} GiB = {:.2} GB\n已使用率: {:.2} %\n",
                rounded_number(parsed_json.monthly_bw_limit_b / 1000000000.0 / 1.073741824),
                rounded_number(parsed_json.monthly_bw_limit_b / 1000000000.0),
                rounded_number(parsed_json.bw_counter_b / 1000000000.0 / 1.073741824),
                rounded_number(parsed_json.bw_counter_b / 1000000000.0),
                rounded_number(parsed_json.monthly_bw_limit_b / 1000000000.0 / 1.073741824 - parsed_json.bw_counter_b / 1000000000.0 / 1.073741824),
                rounded_number(parsed_json.monthly_bw_limit_b / 1000000000.0 - parsed_json.bw_counter_b / 1000000000.0),
                rounded_number(parsed_json.bw_counter_b / parsed_json.monthly_bw_limit_b * 100.0)
            );
            return result;
        }
        Err(e) => {
            return format!("Error parsing JSON: {}", e);
        }
    }
}
