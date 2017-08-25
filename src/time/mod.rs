extern crate chrono;

use chrono::prelude::*;

pub fn utc_to_julian_date(date : &DateTime<Utc>) -> f64 {
    parts_to_julian_date(&date.naive_utc())
}

pub fn local_to_julian_date(date : &DateTime<Local>) -> f64 {
    0f64
}

pub fn fixed_to_julian_date(date : &DateTime<FixedOffset>) -> f64 {
    0f64
}

fn decimal_day(time : &chrono::NaiveTime) -> f64 {
    0f64
}

fn parts_to_julian_date(time : &chrono::NaiveDateTime) -> f64 {
    // JD = INT(365.25*(Y+4716))
    //    + INT(30.6001*(M+1))
    //    + D + B - 1524.5

    let cal_constant = 0f64;

    let year = match time.month() {
        1 | 2 => time.year() - 1,
        _ => time.year()
    } as f64;

    let month = match time.month() {
        1 | 2 => time.month() + 12,
        _ => time.month()
    } as f64;

    (365.25_f64 * (year + 4716_f64)).floor()
    + (30.6001_f64 * (month + 1f64)).floor()
    + decimal_day(&time.time()) + cal_constant - 1524.5_f64
}

#[cfg(test)]
mod tests {
    use time::*;

    #[test]
    fn handles_utc() {
        struct CheckPair { date : DateTime<Utc>, result : f64 };

        let pairs = [
            CheckPair { date: Utc.ymd(2000, 1, 1).and_hms(12, 0, 0), result: 2451545.0f64 }
        ];

        for pair in pairs.iter() {
            assert_eq!(utc_to_julian_date(&pair.date), pair.result);
        }
    }
}