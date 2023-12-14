extern crate chrono;

// Export chrono::Weekday as wd to satisfy the tests without modification
pub use chrono::Weekday as wd;
use chrono::{Datelike, NaiveDate};

pub fn middle_day(year: i32) -> Option<wd> {
    // Leap year check
    let is_leap_year = NaiveDate::from_ymd_opt(year, 2, 29).is_some();

    // If it's a leap year, there's no single middle day
    if is_leap_year {
        return None;
    }

    // Otherwise, we will consider July 2nd as the middle day for non-leap years (day 182 of 365)
    let middle_day = NaiveDate::from_ymd(year, 7, 2).weekday();

    Some(middle_day)
}