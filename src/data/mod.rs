pub mod chargecny;
pub mod dataused;
pub mod exchangerate;

fn rounded_number(number: f64, dot: i32) -> f64 {
    let fix_number = 10.0_f64.powf(dot as f64);
    (number * fix_number).floor() / fix_number
}
