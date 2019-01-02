/// Represents a standard date according to ISO 8601.
#[derive(Copy, Clone)]
pub struct ISO8601Date {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl ISO8601Date {
    /// Indicates whether the month/day pair is valid in a leap year
    pub fn valid_month_day(month: u8, day: u8) -> bool {
        const EXPECTED_MONTHS: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        const EXPECTED_DAYS: &[u8] = &[
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31,
        ];
        const MAX_DAYS: &[u8] = &[0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        EXPECTED_MONTHS.contains(&month)
            && EXPECTED_DAYS.contains(&day)
            && MAX_DAYS[usize::from(month)] >= day
    }
}
