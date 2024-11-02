use std::fs;

use charts_rs::get_font_families;

static mut INSTANCE: Option<Config> = None;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Config {
    pub subscribed_money: f64,
    pub subscribed_count: i64,
    pub expand_rate: f64,
    pub bot_token: String,
    pub group_chat_id: i64,
    pub used_data_url: String,
    pub exchange_rate_url: String,
    pub event_trigger_time: String,
    pub daily_cron_time: String,
    pub max_store_charge_num: usize,
    pub subscribed_id_list: Vec<i64>,
    pub font_path: String,
    pub help_message: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            subscribed_money: 0.0,
            subscribed_count: 0,
            expand_rate: 0.0,
            bot_token: "".to_string(),
            group_chat_id: 0,
            used_data_url: "".to_string(),
            exchange_rate_url: "".to_string(),
            event_trigger_time: "".to_string(),
            daily_cron_time: "".to_string(),
            max_store_charge_num: 1,
            subscribed_id_list: vec![],
            font_path: "".to_string(),
            help_message: "".to_string(),
        }
    }
}

impl Config {
    fn new() -> Config {
        let file_content =
            fs::read_to_string("./config.json").expect("LogRocket: error reading config file");
        match serde_json::from_str::<Config>(&file_content) {
            Ok(parsed_json) => parsed_json,
            Err(_e) => Config {
                ..Default::default()
            },
        }
    }
}

pub fn get_instance() -> &'static Config {
    unsafe {
        if INSTANCE.is_none() {
            INSTANCE = Some(Config::new());
        }
        INSTANCE.as_ref().unwrap()
    }
}

pub fn get_font_family_name() -> String {
    let vec = get_font_families().unwrap();
    for item in vec.iter() {
        if !item.contains("Roboto") {
            return item.to_string();
        }
    }
    "Roboto".to_string()
}
