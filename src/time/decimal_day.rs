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
/// assert_eq!(7.5, meealgi::time::decimal_day(&now.naive_utc()));
/// # }
/// ```
pub fn decimal_day(date : &NaiveDateTime) -> f64 {
    const SECONDS_IN_MINUTE : f64 = 60_f64;
    const SECONDS_IN_HOUR : f64 = 60_f64 * SECONDS_IN_MINUTE;
    const SECONDS_IN_DAY : f64 = 24_f64 * SECONDS_IN_HOUR;

    f64::from(date.day())
    + (f64::from(date.hour()) * SECONDS_IN_HOUR
    + f64::from(date.minute()) * SECONDS_IN_MINUTE
    + f64::from(date.second())) / SECONDS_IN_DAY
}