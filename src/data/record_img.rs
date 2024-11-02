use charts_rs::{svg_to_jpeg, LineChart};

use crate::{
    data::rounded_number,
    tools::{config::get_font_family_name, history::CheckInstance},
};

pub fn check2vecstr(vec: &Vec<CheckInstance>) -> ((String, String, String), f64) {
    let byte2gb = 1000000000.0;
    let mut today_used_data_vec = vec![];
    let mut total_used_data_vec = vec![];
    let mut max_data_vec = vec![];
    let mut date_vec = vec![];
    vec.iter().for_each(|check| {
        today_used_data_vec.push(rounded_number(check.today_used_data / byte2gb, 2));
        total_used_data_vec.push(rounded_number(check.total_used_data / byte2gb, 2));
        max_data_vec.push(rounded_number(check.max_data / byte2gb, 2));
        date_vec.push(check.date.clone());
    });
    (
        (
            format! {"{:?}", today_used_data_vec},
            format! {"{:?}", total_used_data_vec},
            format! {"{:?}", date_vec},
        ),
        max_data_vec[0],
    )
}

pub fn get_record_img(vec: &Vec<CheckInstance>) -> anyhow::Result<Vec<u8>> {
    let ((daily_gb, total_gb, date), _) = check2vecstr(vec);
    let raw_json = r###"{
                                "font_family": "{{font_family}}",
                                "height": 800,
                                "legend_align": "right",
                                "legend_category": "round rect",
                                "legend_font_size": 16,
                                "legend_margin": {
                                "left": 0,
                                "top": 5,
                                "right": 0,
                                "bottom": 0
                                },
                                "legend_show": true,
                                "margin": {
                                "left": 15,
                                "top": 10,
                                "right": 50,
                                "bottom": 5
                                },
                                "quality": 80,
                                "series_list": [
                                {
                                    "name": "Daily GB",
                                    "label_show": true,
                                    "data": {{daily_gb}},
                                    "mark_points": [
                                    {
                                        "category": "max"
                                    },
                                    {
                                        "category": "min"
                                    }
                                    ],
                                    "mark_lines": [
                                    {
                                        "category": "average"
                                    }
                                    ]
                                },
                                {
                                    "name": "Total GB",
                                    "data": {{total_gb}},
                                    "mark_points": [
                                    {
                                        "category": "max"
                                    }
                                    ]
                                }
                                ],
                                "series_smooth": true,
                                "sub_title_align": "left",
                                "sub_title_font_size": 18,
                                "sub_title_height": 24,
                                "sub_title_margin": {
                                    "left": 0,
                                    "top": 0,
                                    "right": 0,
                                    "bottom": 0
                                },
                                "sub_title_text": "按流量重置后日期",
                                "theme": "grafana",
                                "title_align": "left",
                                "title_font_size": 24,
                                "title_font_weight": "bold",
                                "title_height": 30,
                                "title_margin": {
                                "left": 0,
                                "top": 0,
                                "right": 0,
                                "bottom": 0
                                },
                                "title_text": "JMS 流量使用",
                                "type": "line",
                                "width": 1200,
                                "x_axis_data": {{date}},
                                "x_axis_font_size": 18,
                                "x_axis_height": 30,
                                "x_axis_hidden": false,
                                "x_axis_margin": {
                                "left": 0,
                                "top": 0,
                                "right": 0,
                                "bottom": 0
                                },
                                "x_axis_name_gap": 5,
                                "x_axis_name_rotate": 0,
                                "x_boundary_gap": true,
                                "y_axis_hidden": false
                            }"###;
    println!("{:?}", raw_json);
    let formatted_json = raw_json
        .replace("{{font_family}}", &get_font_family_name())
        .replace("{{daily_gb}}", &daily_gb)
        .replace("{{total_gb}}", &total_gb)
        .replace("{{date}}", &date);

    let line_chart = LineChart::from_json(&formatted_json)?;
    Ok(svg_to_jpeg(&line_chart.svg().unwrap()).unwrap())
}
