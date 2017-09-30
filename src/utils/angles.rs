//! Utilities for working with angles, such as limiting
//! angles to a specific range
const TWO_PI: f64 = ::std::f64::consts::PI * 2_f64;

/// Limits a value into the range 0 -> 2*PI
///
/// # Examples:
/// ```
/// use meealgi::utils::angles::limit_radians;
/// use std::f64::consts::*;
///
/// # fn main() {
/// assert_eq!(0_f64, limit_radians(-PI * 2_f64));
/// assert_eq!(0_f64, limit_radians(PI * 2_f64));
/// assert_eq!(PI, limit_radians(PI * 3_f64));
/// assert_eq!(FRAC_PI_4 * 7_f64, limit_radians(-FRAC_PI_4));
/// # }
/// ```
pub fn limit_radians(rad: f64) -> f64 {
    let revs = rad / TWO_PI;
    TWO_PI * (revs - revs.floor())
}

#[cfg(test)]
mod tests {
    use utils::angles::limit_radians;
    use std::f64::consts::{PI, FRAC_PI_4};
    use spectral::prelude::*;

    #[test]
    fn handles_positive() {
        assert_that!(limit_radians(PI * 2_f64)).is_close_to(0_f64, 0.00001_f64);
        assert_that!(limit_radians(PI * 3_f64)).is_close_to(PI, 0.00001_f64);
        assert_that!(limit_radians(FRAC_PI_4)).is_close_to(FRAC_PI_4, 0.00001_f64);
        assert_that!(limit_radians(PI * 3_f64 + FRAC_PI_4))
            .is_close_to(PI + FRAC_PI_4, 0.00001_f64);
        assert_that!(limit_radians(PI * 4_f64 + FRAC_PI_4)).is_close_to(FRAC_PI_4, 0.00001_f64);
    }

    #[test]
    fn handles_negative() {
        assert_that!(limit_radians(-PI * 2_f64)).is_close_to(0_f64, 0.00001_f64);
        assert_that!(limit_radians(-PI * 3_f64)).is_close_to(PI, 0.00001_f64);
        assert_that!(limit_radians(-FRAC_PI_4)).is_close_to(2_f64 * PI - FRAC_PI_4, 0.00001_f64);
        assert_that!(limit_radians(-PI * 3_f64 - FRAC_PI_4))
            .is_close_to(PI - FRAC_PI_4, 0.00001_f64);
        assert_that!(limit_radians(-PI * 4_f64 + FRAC_PI_4)).is_close_to(FRAC_PI_4, 0.00001_f64);
    }
}
