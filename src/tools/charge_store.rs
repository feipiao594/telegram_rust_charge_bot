use chrono::{Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::Path,
};

use super::config;

#[derive(Serialize, Deserialize)]
struct StoreItem {
    date: String,
    charge_str: String,
}

pub async fn push_charge_store(map: (String, String)) -> std::io::Result<()> {
    let (_, charge_str) = map;

    let now = Utc::now();
    let (is_pm, hour) = now.hour12();
    let (_, year) = now.year_ce();
    let date = format!(
        "(UTC) {}-{:02}-{:02} {:02}:{:02}:{:02} {}",
        year,
        now.month(),
        now.day(),
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );

    let item = StoreItem {
        date: date.to_string(),
        charge_str: charge_str.to_string(),
    };

    if !Path::new("history.json").exists() {
        File::create("history.json").expect("Error: cannot create history file");
    }

    let contents = fs::read_to_string("./history.json").expect("Error: read history file");

    let mut items: Vec<StoreItem> = if contents.trim().is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents)?
    };

    let n = items.len();
    let max_store_num = config::get_instance().max_store_charge_num;

    if n >= max_store_num {
        for _ in 0..n - max_store_num {
            items.remove(0);
        }
    }

    items.push(item);

    let updated_content = serde_json::to_string_pretty(&items)?;

    fs::write("./history.json", updated_content).expect("Error: write history file");

    Ok(())
}

pub async fn get_recent_store() -> std::io::Result<String> {
    if !Path::new("history.json").exists() {
        File::create("history.json").expect("Error: cannot create history file");
    }

    let contents = fs::read_to_string("./history.json").expect("Error: read history file");

    let items: Vec<StoreItem> = if contents.trim().is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&contents)?
    };

    let n = items.len();
    if n == 0 {
        return Ok("没有最近的记录".to_string());
    }

    let ret = format!(
        "以下为最近的记录，记录日期为{}\n\n{}",
        items[n - 1].date,
        items[n - 1].charge_str
    );
    Ok(ret)
}
