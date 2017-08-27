extern crate chrono;

use chrono::{NaiveDateTime, Datelike, Timelike};

/// Convert a `chrono::NaiveDateTime` to a decimal day
///
/// # Examples
///
/// ```rust
/// # extern crate meealgi;
/// # extern crate chrono;
/// # use chrono::TimeZone;
/// # fn main() {
/// let now = chrono::Utc.ymd(2017,01,07).and_hms(12, 00, 00);
/// meealgi::time::decimal_day(&now.naive_utc());
/// // -> 7.5
/// # }
/// ```
pub fn decimal_day(date : &NaiveDateTime) -> f64 {
    const SECONDS_IN_MINUTE : f64 = 60_f64;
    const SECONDS_IN_HOUR : f64 = 60_f64 * SECONDS_IN_MINUTE;
    const SECONDS_IN_DAY : f64 = 24_f64 * SECONDS_IN_HOUR;

    date.day() as f64
    + (date.hour() as f64 * SECONDS_IN_HOUR
    + date.minute() as f64 * SECONDS_IN_MINUTE
    + date.second() as f64) / SECONDS_IN_DAY
}