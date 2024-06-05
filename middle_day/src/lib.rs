extern crate chrono;
pub use chrono::{Datelike, NaiveDate, Weekday as wd};
// Use the chrono crate to create a function named middle_day. It accepts a year, and returns the weekday of the middle day of that year, wrapped in an Option. chrono::Weekday has to be referred to as wd.

// Years with an even number of days do not have a middle day, and should return None.
#[warn(deprecated)]
pub fn middle_day(year : i32) -> Option<wd>{
    let mut date = NaiveDate::from_ymd_opt(year, 1, 1)?;
    if date.weekday().number_from_monday() % 2 == 0 {
        return None;
    }
    date = date.succ_opt()?;
    while date.weekday().number_from_monday() % 2!= 0 {
        date = date.succ_opt()?;
    }
    Some(date.weekday())
}
