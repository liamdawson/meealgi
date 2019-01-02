mod date;
mod standard;

pub use self::date::*;
pub use self::standard::StandardCalendar;

/// Functions common to all ISO 8601 calendar implementations
pub trait ISO8601Calendar {
    /// Obtain a validated date for the implementing calendar
    fn ymd(year: i32, month: u8, day: u8) -> Option<self::ISO8601Date>;
    /// Check if the given year is a leap year (returning None if the year is invalid)
    fn is_leap_year(year: i32) -> Option<bool>;
}
