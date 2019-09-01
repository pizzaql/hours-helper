extern crate chrono;

use chrono::{DateTime, Local, Timelike};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn get_time(number: u32, is_half_past: bool) -> String {
    let now: DateTime<Local> = Local::now();
    let date: u32 = now.hour() + number;

    if is_half_past {
        if date == 24 {
            "00:30".to_string()
        } else {
            format!("{}:30", date)
        }
    } else if date == 24 {
        "00:30".to_string()
    } else {
        format!("{}:00", date)
    }
}
