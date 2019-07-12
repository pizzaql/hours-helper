extern crate chrono;

use wasm_bindgen::prelude::*;
use chrono::{DateTime, Timelike, Local};

#[wasm_bindgen]
pub fn get_time(number: u32, is_half_past: bool) -> String {
    let now: DateTime<Local> = Local::now();
    let date: u32 = now.hour() + number;

    if is_half_past {
        format!("{}:30", date)
    } else {
        format!("{}:00", date)
    }
}