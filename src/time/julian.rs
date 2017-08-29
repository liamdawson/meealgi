extern crate chrono;

use chrono::{NaiveDateTime, Datelike};
use time::decimal_day;

/// Convert a `chrono::NaiveDateTime` to a decimal Julian Day
///
/// # Examples
///
/// ```rust
/// # extern crate meealgi;
/// # extern crate chrono;
/// # use chrono::TimeZone;
/// # fn main() {
/// let now = chrono::Utc.ymd(2017,01,01).and_hms(00, 00, 00);
/// meealgi::time::ndt_to_jul(&now.naive_utc());
/// # }
/// ```
pub fn ndt_to_jul(date : &NaiveDateTime) -> f64 {
    let year = match date.month() {
        1 | 2 => date.year() - 1,
        _ => date.year()
    } as f64;

    let month = match date.month() {
        1 | 2 => date.month() + 12,
        _ => date.month()
    } as f64;

    let pre_shift_val = (365.25_f64 * (year + 4716_f64)).floor()
    + (30.6001_f64 * (month + 1f64)).floor()
    + decimal_day(&date) - 1524.5_f64;

    let gregorian_shift_factor = match pre_shift_val > 2299160f64 {
        false => 0f64,
        true => {
            let year_factor = (year / 100f64).floor();
            2f64 - year_factor + (year_factor / 4f64).floor()
        }
    };

    pre_shift_val + gregorian_shift_factor
}

/// Calculate the Julian century from a Julian Day.
///
/// # Examples
/// ```rust
/// # extern crate chrono;
/// # extern crate meealgi;
/// # use chrono::prelude::*;
/// # use meealgi::time::*;
/// # fn main() {
/// let jd = ndt_to_jul(&chrono::NaiveDate::from_ymd(2017, 08, 29).and_hms(12, 00, 00));
/// let jc = jul_to_julc(jd);
/// # }
/// ```
pub fn jul_to_julc(jd : f64) -> f64 {
    (jd - 2451545_f64) / 36525_f64
}

/// Calculate the Julian millenium from a Julian century
///
/// # Examples
/// ```rust
/// # extern crate chrono;
/// # extern crate meealgi;
/// # use chrono::prelude::*;
/// # use meealgi::time::*;
/// # fn main() {
/// let day = ndt_to_jul(&chrono::NaiveDate::from_ymd(2017, 08, 30).and_hms(12, 00, 00));
/// let jul_century = jul_to_julc(day);
/// let jul_mil = julc_to_julm(jul_century);
/// # }
/// ```
pub fn julc_to_julm(jc : f64) -> f64 {
    jc / 10_f64
}

#[cfg(test)]
mod tests {
    use chrono::prelude::{TimeZone, Utc, DateTime};
    use time::*;

    #[test]
    fn jd_solar_report_tests() {
        struct CheckPair { date : DateTime<Utc>, result : f64 };

        // test data from NREL Solar A.4.1
        let pairs = [
            CheckPair { date: Utc.ymd( 2000, 01, 01).and_hms(12, 00, 00), result: 2451545.0_f64 },
            CheckPair { date: Utc.ymd( 1999, 01, 01).and_hms(00, 00, 00), result: 2451179.5_f64 },
            CheckPair { date: Utc.ymd( 1987, 01, 27).and_hms(00, 00, 00), result: 2446822.5_f64 },
            CheckPair { date: Utc.ymd( 1987, 06, 19).and_hms(12, 00, 00), result: 2446966.0_f64 },
            CheckPair { date: Utc.ymd( 1988, 01, 27).and_hms(00, 00, 00), result: 2447187.5_f64 },
            CheckPair { date: Utc.ymd( 1988, 06, 19).and_hms(12, 00, 00), result: 2447332.0_f64 },
            CheckPair { date: Utc.ymd( 1900, 01, 01).and_hms(00, 00, 00), result: 2415020.5_f64 },
            CheckPair { date: Utc.ymd( 1600, 01, 01).and_hms(00, 00, 00), result: 2305447.5_f64 },
            CheckPair { date: Utc.ymd( 1600, 12, 31).and_hms(00, 00, 00), result: 2305812.5_f64 },
            CheckPair { date: Utc.ymd( 0837, 04, 10).and_hms(07, 12, 00), result: 2026871.8_f64 },
            CheckPair { date: Utc.ymd(-0123, 12, 31).and_hms(00, 00, 00), result: 1676496.5_f64 },
            CheckPair { date: Utc.ymd(-0122, 01, 01).and_hms(00, 00, 00), result: 1676497.5_f64 },
            CheckPair { date: Utc.ymd(-1000, 07, 12).and_hms(12, 00, 00), result: 1356001.0_f64 },
            // TODO(vendor-issue): chronotope/chrono#180
            // CheckPair { date: Utc.ymd(-1000, 02, 29).and_hms(00, 00, 00), result: 1355866.5_f64 },
            CheckPair { date: Utc.ymd(-1001, 08, 17).and_hms(21, 36, 00), result: 1355671.4_f64 },
            CheckPair { date: Utc.ymd(-4712, 1, 1).and_hms(12, 00, 00), result: 0.0_f64 },
        ];

        for pair in pairs.iter() {
            assert_eq!(pair.result, ndt_to_jul(&pair.date.naive_utc()));
        }
    }
}