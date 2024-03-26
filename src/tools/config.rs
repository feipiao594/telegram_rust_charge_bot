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
}

impl Config {
    fn new() -> Config {
        // let file_content =
        //     fs::read_to_string("./config.json").expect("LogRocket: error reading file");
        Config {
            subscribed_money: 1.0,
            expand_rate: 1.0,
            subscribed_count: 1,
            bot_token: "114514".to_string(),
            group_chat_id: 114514,
            used_data_url: "https://www.baidu.com".to_string(),
            exchange_rate_url: "https://www.baidu.com".to_string(),
            event_trigger_time: "1/10 0* * * * ?".to_string(),
        }
    }
}

pub fn get_instance() -> &'static Config {
    unsafe {
        if INSTANCE.is_none() {
            INSTANCE = Some(Config::new());
        }
        &INSTANCE.as_ref().unwrap()
    }
}
