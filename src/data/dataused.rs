use crate::data::rounded_number;
use crate::tools::{config, http};
use serde;

#[derive(Debug, Clone, serde::Deserialize)]
struct DataUsed {
    monthly_bw_limit_b: f64,
    bw_counter_b: f64,
}

pub async fn get_used_data() -> String {
    let url = &config::get_instance().used_data_url;
    let body = http::url_get(&url).await;
    // println!("{}", body);
    let gb2gib = 1.073741824;
    let byte2gb = 1000000000.0;
    match serde_json::from_str::<DataUsed>(&body) {
        Ok(parsed_json) => {
            let result = format!(
                "每月总额: {:.2} GiB = {:.2} GB\n已用流量: {:.2} GiB = {:.2} GB\n剩余流量: {:.2} GiB = {:.2} GB\n已使用率: {:.2} %\n",
                rounded_number(parsed_json.monthly_bw_limit_b / byte2gb / gb2gib,2),
                rounded_number(parsed_json.monthly_bw_limit_b / byte2gb,2),
                rounded_number(parsed_json.bw_counter_b / byte2gb / gb2gib,2),
                rounded_number(parsed_json.bw_counter_b / byte2gb,2),
                rounded_number(parsed_json.monthly_bw_limit_b / byte2gb / gb2gib - parsed_json.bw_counter_b / byte2gb / gb2gib,2),
                rounded_number(parsed_json.monthly_bw_limit_b / byte2gb - parsed_json.bw_counter_b / byte2gb,2),
                rounded_number(parsed_json.bw_counter_b / parsed_json.monthly_bw_limit_b * 100.0,2)
            );
            return result;
        }
        Err(e) => {
            return format!("Error parsing JSON: {}", e);
        }
    }
}
