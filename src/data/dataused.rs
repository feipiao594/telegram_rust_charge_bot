use crate::data::rounded_number;
use crate::tools::config::get_font_family_name;
use crate::tools::{config, http};
use charts_rs::{svg_to_jpeg, PieChart};
use serde;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DataUsed {
    pub monthly_bw_limit_b: f64,
    pub bw_counter_b: f64,
    pub bw_reset_day_of_month: f64,
}

pub async fn get_data_used() -> anyhow::Result<DataUsed> {
    let url = &config::get_instance().used_data_url;
    let body = http::url_get(url).await;
    log::info!("request used data");
    // println!("{}", body);

    let m_data_used = serde_json::from_str::<DataUsed>(&body).unwrap();
    // println!("{:?}", m_data_used);
    Ok(m_data_used)
}

pub async fn get_used_data(m_data_used: DataUsed) -> String {
    let gb2gib = 1.073741824;
    let byte2gb = 1000000000.0;
    let _ = m_data_used.bw_reset_day_of_month;
    let result = format!(
                "每月总额: {:.2} GiB = {:.2} GB\n已用流量: {:.2} GiB = {:.2} GB\n剩余流量: {:.2} GiB = {:.2} GB\n已使用率: {:.2} %\n",
                rounded_number(m_data_used.monthly_bw_limit_b / byte2gb / gb2gib,2),
                rounded_number(m_data_used.monthly_bw_limit_b / byte2gb,2),
                rounded_number(m_data_used.bw_counter_b / byte2gb / gb2gib,2),
                rounded_number(m_data_used.bw_counter_b / byte2gb,2),
                rounded_number(m_data_used.monthly_bw_limit_b / byte2gb / gb2gib - m_data_used.bw_counter_b / byte2gb / gb2gib,2),
                rounded_number(m_data_used.monthly_bw_limit_b / byte2gb - m_data_used.bw_counter_b / byte2gb,2),
                rounded_number(m_data_used.bw_counter_b / m_data_used.monthly_bw_limit_b * 100.0,2)
            );
    result
}

pub async fn get_used_data_img(m_data_used: DataUsed) -> anyhow::Result<Vec<u8>> {
    let byte2gb = 1000000000.0;
    let raw_json = r###"{
            "border_radius": 8,
            "font_family": "{{font_family}}",
            "height": 400,
            "inner_radius": 30,
            "legend_align": "right",
            "legend_category": "normal",
            "legend_font_size": 16,
            "legend_margin": {
                "left": 5,
                "top": 10,
                "right": 15,
                "bottom": 5
            },
            "legend_show": true,
            "margin": {
                "left": 15,
                "top": 10,
                "right": 5,
                "bottom": 5
            },
            "quality": 80,
            "radius": 110,
            "rose_type": false,
            "series_list": [
                {
                "name": "Used",
                "data": [
                    {{Used}}
                ]
                },
                {
                "name": "Unused",
                "data": [
                    {{Unused}}
                ]
                }
            ],
            "sub_title_align": "left",
            "sub_title_font_size": 16,
            "sub_title_height": 20,
            "sub_title_margin": {
                "left": 15,
                "top": 10,
                "right": 5,
                "bottom": 5
            },
            "sub_title_text": "从上个计费日开始",
            "theme": "grafana",
            "title_align": "left",
            "title_font_size": 24,
            "title_font_weight": "bold",
            "title_height": 30,
            "title_margin": {
                "left": 15,
                "top": 10,
                "right": 5,
                "bottom": 5
            },
            "title_text": "JMS 流量使用",
            "type": "pie",
            "width": 600,
            "x_axis_font_size": 16,
            "x_axis_height": 30
        }"###;
    println!("{}", get_font_family_name());
    let formatted_json = raw_json
        .replace("{{font_family}}", &get_font_family_name())
        .replace(
            "{{Used}}",
            &format!("{}", rounded_number(m_data_used.bw_counter_b / byte2gb, 2)),
        )
        .replace(
            "{{Unused}}",
            &format!(
                "{}",
                rounded_number(
                    m_data_used.monthly_bw_limit_b / byte2gb - m_data_used.bw_counter_b / byte2gb,
                    2,
                )
            ),
        );
    println!("{}", formatted_json);
    let bar_chart = PieChart::from_json(&formatted_json)?;
    Ok(svg_to_jpeg(&bar_chart.svg().unwrap()).unwrap())
}
