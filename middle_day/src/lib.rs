use chrono::{NaiveDate, Weekday};
use chrono::Datelike;
pub fn middle_day(year: u32) -> Option<Weekday> {
    let year = year as i32;
    let start = NaiveDate::from_ymd_opt(year, 1, 1)?;
    let end   = NaiveDate::from_ymd_opt(year + 1, 1, 1)?;
    let days  = (end - start).num_days();

    if days % 2 == 0 {
        None
    } else {
        Some((start + (end - start) / 2).weekday())
    }
}