//! Utilities for working with angles, such as limiting
//! angles to a specific range
 
use std::f64::consts::PI;

/// Limits a value into the range 0 -> 2*PI
///
/// # Examples:
/// ```
/// use meealgi::utils::angles::limit_radians;
/// use std::f64::consts::*;
///
/// # fn main() {
/// assert_eq!(0_f64, limit_radians(PI * 200_f64));
/// # }
/// ```
pub fn limit_radians(val: f64) -> f64 {
    const LIMIT : f64 = 2_f64 * PI;

    let proportion = val / LIMIT;

    let fractional = proportion - proportion.trunc();
    let result = fractional * LIMIT;

    if result < 0_f64 {
        result + LIMIT
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use utils::angles::limit_radians;
    use std::f64::consts::{PI, FRAC_PI_4};
    use spectral::prelude::*;

    #[test]
    fn handles_positive() {
        assert_that!(limit_radians(0_f64)).is_close_to(0_f64, 0.00000001_f64);
        assert_that!(limit_radians(2_f64 * PI)).is_close_to(0_f64, 0.00000001_f64);
        assert_that!(limit_radians(3_f64 * PI)).is_close_to(PI, 0.00000001_f64);
        assert_that!(limit_radians(5_f64 * PI)).is_close_to(PI, 0.00000001_f64);
        assert_that!(limit_radians(FRAC_PI_4)).is_close_to(FRAC_PI_4, 0.00000001_f64);
        assert_that!(limit_radians(PI + FRAC_PI_4)).is_close_to(PI + FRAC_PI_4, 0.00000001_f64);
        assert_that!(limit_radians(100_f64 * PI)).is_close_to(0_f64, 0.00000001_f64);
    }

    #[test]
    fn handles_negative() {
        assert_that!(limit_radians(-2_f64 * PI)).is_close_to(0_f64, 0.00000001_f64);
        assert_that!(limit_radians(-3_f64 * PI)).is_close_to(PI, 0.00000001_f64);
        assert_that!(limit_radians(-5_f64 * PI)).is_close_to(PI, 0.00000001_f64);
        assert_that!(limit_radians(-FRAC_PI_4)).is_close_to(2_f64 * PI - FRAC_PI_4, 0.00000001_f64);
        assert_that!(limit_radians(-PI - FRAC_PI_4)).is_close_to(3_f64 * FRAC_PI_4, 0.00000001_f64);
        assert_that!(limit_radians(-PI + FRAC_PI_4)).is_close_to(PI + FRAC_PI_4, 0.00000001_f64);
        assert_that!(limit_radians(-100_f64 * PI)).is_close_to(0_f64, 0.00000001_f64);
    }
}
