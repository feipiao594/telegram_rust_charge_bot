use chrono::{Datelike, Timelike, Utc};
use log::info;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    path::Path,
};

use crate::data::dataused::DataUsed;

use super::config;

#[derive(Serialize, Deserialize, Debug)]
struct StoreItem {
    date: String,
    charge_str: String,
    last_checks: Vec<CheckInstance>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct History {
    current_checks: Vec<CheckInstance>,
    items: Vec<StoreItem>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CheckInstance {
    pub today_used_data: f64,
    pub total_used_data: f64,
    pub max_data: f64,
    pub date: String,
}

pub fn get_current_checks_number(his: &History) -> i64 {
    his.current_checks.len() as i64
}

pub fn get_current_checks(his: &History) -> &Vec<CheckInstance> {
    &his.current_checks
}

pub fn get_recent_checks_number(his: &History) -> i64 {
    let n = his.items.len();
    if n == 0 {
        return 0;
    }
    his.items[n - 1].last_checks.len() as i64
}

pub fn get_recent_checks(his: &History) -> &Vec<CheckInstance> {
    let n = his.items.len();
    &his.items[n - 1].last_checks
}

pub async fn get_history() -> std::io::Result<History> {
    if !Path::new("history.json").exists() {
        File::create("history.json").expect("Error: cannot create history file");
    }

    let contents = fs::read_to_string("./history.json").expect("Error: read history file");
    if contents.trim().is_empty() {
        info!("history file is empty, create new json");
        return Ok(History {
            current_checks: Vec::new(),
            items: Vec::new(),
        });
    }
    let result = serde_json::from_str(&contents)?;

    Ok(result)
}

async fn push_history(store: History) -> std::io::Result<()> {
    let updated_content = serde_json::to_string_pretty(&store)?;
    fs::write("./history.json", updated_content).expect("Error: write history file");
    Ok(())
}

async fn change_history<HistoryEditorFn>(f: HistoryEditorFn) -> std::io::Result<()>
where
    HistoryEditorFn: FnOnce(History) -> anyhow::Result<History>,
{
    let mut store = get_history().await?;
    let max_store_num = config::get_instance().max_store_charge_num;
    let n = store.items.len();
    if n >= max_store_num {
        for _ in 0..n - max_store_num {
            store.items.remove(0);
        }
    }
    push_history(f(store).unwrap()).await
}

pub async fn append_checks(data_used: DataUsed) -> std::io::Result<()> {
    change_history(|old_history: History| -> anyhow::Result<History> {
        let mut store = old_history;
        let last_used_data = match store.current_checks.last() {
            Some(last) => last.total_used_data,
            None => 0.0,
        };
        let now = Utc::now();
        let check = CheckInstance {
            today_used_data: data_used.bw_counter_b - last_used_data,
            total_used_data: data_used.bw_counter_b,
            max_data: data_used.monthly_bw_limit_b,
            date: format!("{}.{}", now.month(), now.day()),
        };
        store.current_checks.push(check);
        Ok(store)
    })
    .await
}

pub async fn new_period(charge_str: &String) -> std::io::Result<()> {
    change_history(|old_history: History| -> anyhow::Result<History> {
        let mut store = old_history;
        let now = Utc::now();
        let (is_pm, hour) = now.hour12();
        let (_, year) = now.year_ce();
        let new_item = StoreItem {
            date: format!(
                "(UTC) {}-{:02}-{:02} {:02}:{:02}:{:02} {}",
                year,
                now.month(),
                now.day(),
                hour,
                now.minute(),
                now.second(),
                if is_pm { "PM" } else { "AM" }
            ),
            charge_str: charge_str.clone(),
            last_checks: store.current_checks.clone(),
        };
        store.items.push(new_item);
        store.current_checks.clear();
        Ok(store)
    })
    .await
}

pub async fn get_recent_last_history() -> std::io::Result<String> {
    let items = get_history().await?.items;
    let n = items.len();
    if n == 0 {
        return Ok("暂无记录".to_owned());
    }
    let ret = format!(
        "以下为最近的记录，记录日期为{}\n{}",
        items[n - 1].date,
        items[n - 1].charge_str
    );
    Ok(ret)
}
