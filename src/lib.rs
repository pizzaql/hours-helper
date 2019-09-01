extern crate chrono;

use chrono::{DateTime, Duration, Local, Timelike};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_time(number: u32, is_half_past: bool) -> String {
    let now: DateTime<Local> = Local::now() + Duration::hours(i64::from(number));
    let date = now.hour();

    if is_half_past {
        if date == 0 {
            format!("0{}:30", date)
        } else {
            format!("{}:30", date)
        }
    } else if date == 0 {
        format!("0{}:00", date)
    } else {
        format!("{}:00", date)
    }
}
