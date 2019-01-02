use super::ISO8601Calendar;

/// Represents the minimal standard variant of the ISO 8601 calendar.
///
/// As per ISO 8601, this does not support years prior to 1583, or after 9999.
pub struct StandardCalendar {}

impl super::ISO8601Calendar for StandardCalendar {
    /// Obtains a date instance for valid dates starting from Jan 1 1583.
    fn ymd(year: i32, month: u8, day: u8) -> Option<super::ISO8601Date> {
        if is_valid_date(year, month, day) {
            Some(super::ISO8601Date { year, month, day })
        } else {
            None
        }
    }

    fn is_leap_year(year: i32) -> Option<bool> {
        if !is_valid_year(year) {
            None
        } else {
            // year is divisible by 4, and is either divisible by 400, or isn't divisible by 100
            Some(year % 4 == 0 && (year % 400 == 0 || year % 100 != 0))
        }
    }
}

fn is_valid_year(year: i32) -> bool {
    year >= 1583 && year <= 9999
}

fn is_valid_date(year: i32, month: u8, day: u8) -> bool {
    is_valid_year(year)
        && super::ISO8601Date::valid_month_day(month, day)
        && ((month != 2 || day != 29) || StandardCalendar::is_leap_year(year).unwrap())
}
