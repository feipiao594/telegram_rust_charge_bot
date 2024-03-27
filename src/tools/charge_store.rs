use serde::{Deserialize, Serialize};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
};

use super::config;

#[derive(Serialize, Deserialize)]
struct StoreItem {
    date: String,
    charge_str: String,
}

pub async fn push_charge_store(map: (String, String)) -> std::io::Result<()> {
    let (date, charge_str) = map;

    let item = StoreItem {
        date: date.to_string(),
        charge_str: charge_str.to_string(),
    };

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .append(true)
        .open("store.json")?;

    let mut contents = "".to_string();
    file.read_to_string(&mut contents).unwrap();

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
    file.set_len(0).unwrap();
    file.write_all(updated_content.as_bytes())?;

    Ok(())
}

pub async fn get_recent_store() -> std::io::Result<String> {
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open("store.json")?;

    let mut contents = "".to_string();
    file.read_to_string(&mut contents).unwrap();

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
