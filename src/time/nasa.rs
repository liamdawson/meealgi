//! Time functions published by NASA such as ΔT estimations.
extern crate chrono;

use chrono::{NaiveDate, Datelike};

/// Estimates the ΔT (TD - UT) for a given month and year.
///
/// Implementation of the equations from
/// [NASA](https://web.archive.org/web/20170827023005/https://eclipse.gsfc.nasa.gov/SEcat5/deltatpoly.html).
///
/// # Examples
///
/// ```rust
/// # extern crate chrono;
/// # extern crate meealgi;
/// # fn main() {
/// // day isn't used by the algorithm, but must be valid
/// let month = chrono::NaiveDate::from_ymd(2017, 08, 01);
/// let delta_t = meealgi::time::nasa::delta_t(&month);
/// # }
/// ```
pub fn delta_t(date : &NaiveDate) -> f64 {
    delta_t_frac_year(date.year() as f64
        + (date.month() as f64 - 0.5_f64) / 12_f64)
}

fn delta_t_frac_year(y : f64) -> f64 {
    if y < -500_f64 || y >= 2150_f64 {
        let u = (y - 1820_f64) / 100_f64;

        -20_f64
        + 32_f64 * u.powi(2)
    } else if y < 500_f64 {
        let u = y / 100_f64;

	    10583.6_f64
        - 1014.41_f64 * u
        + 33.78311_f64 * u.powi(2)
        - 5.952053_f64 * u.powi(3)
		- 0.1798452_f64 * u.powi(4)
        + 0.022174192_f64 * u.powi(5)
        + 0.0090316521_f64 * u.powi(6)
    } else if y < 1600_f64 {
        let u = (y - 1000_f64) / 100_f64;

	    1574.2_f64
        - 556.01_f64 * u
        + 71.23472_f64 * u.powi(2)
        + 0.319781_f64 * u.powi(3)
		- 0.8503463_f64 * u.powi(4)
        - 0.005050998_f64 * u.powi(5)
        + 0.0083572073_f64 * u.powi(6)
    } else if y < 1700_f64 {
        let u = y - 1600_f64;

	    120_f64
        - 0.9808_f64 * u
        - 0.01532_f64 * u.powi(2)
        + u.powi(3) / 7129_f64
    } else if y < 1800_f64 {
        let u = y - 1700_f64;

        8.83_f64
        + 0.1603_f64 * u
        - 0.0059285_f64 * u.powi(2)
        + 0.00013336_f64 * u.powi(3)
        - u.powi(4) / 1174000_f64
    } else if y < 1860_f64 {
        let u = y - 1800_f64;

        13.72_f64
        - 0.332447_f64 * u
        + 0.0068612_f64 * u.powi(2)
        + 0.0041116_f64 * u.powi(3)
        - 0.00037436_f64 * u.powi(4)
        + 0.0000121272_f64 * u.powi(5)
        - 0.0000001699_f64 * u.powi(6)
        + 0.000000000875_f64 * u.powi(7)
    } else if y < 1900_f64 {
        let u = y - 1860_f64;

	    7.62_f64
        + 0.5737_f64 * u
        - 0.251754_f64 * u.powi(2)
        + 0.01680668_f64 * u.powi(3)
    } else if y < 1920_f64 {
        let u = y - 1900_f64;

	    -2.79_f64
        + 1.494119_f64 * u
        - 0.0598939_f64 * u.powi(2)
        + 0.0061966_f64 * u.powi(3)
        - 0.000197_f64 * u.powi(4)
    } else if y < 1941_f64 {
        let u = y - 1920_f64;

	    21.20_f64
        + 0.84493_f64 * u
        - 0.076100_f64 * u.powi(2)
        + 0.0020936_f64 * u.powi(3)
    } else if y < 1961_f64 {
        let u = y - 1950_f64;

	    29.07_f64
        + 0.407_f64 * u
        - u.powi(2) / 233_f64
        + u.powi(3) / 2547_f64
    } else if y < 1986_f64 {
        let u = y - 1975_f64;

	    45.45_f64
        + 1.067_f64 * u
        - u.powi(2) / 260_f64
        - u.powi(3) / 718_f64
    } else if y < 2005_f64 {
        let u = y - 2000_f64;

	    63.86_f64
        + 0.3345_f64 * u
        - 0.060374_f64 * u.powi(2)
        + 0.0017275_f64 * u.powi(3)
        + 0.000651814_f64 * u.powi(4) 
        + 0.00002373599_f64 * u.powi(5)
    } else if y < 2050_f64 {
        let u = y - 2000_f64;
        62.92_f64
        + 0.32217_f64 * u
        + 0.005589_f64 * u.powi(2)
    } else if y < 2150_f64 {
        -20_f64
        + 32_f64 * ((y - 1820_f64)/100_f64).powi(2)
        - 0.5628_f64 * (2150_f64 - y)
    } else {
        // handled by first condition
        unreachable!()
    }
}

/// Estimates the Julian Ephemeris Day for a given datetime.
///
/// # Examples
///
/// ```rust
/// # extern crate chrono;
/// # extern crate meealgi;
/// # use chrono::prelude::*;
/// # use meealgi::time::nasa::*;
/// # fn main () {
/// let jule = ndt_to_jule(&chrono::NaiveDate::from_ymd(2017, 08, 29).and_hms(12, 00, 00));
/// # }
/// ```
pub fn ndt_to_jule(ndt : &chrono::NaiveDateTime) -> f64 {
    ::time::ndt_to_jul(&ndt) + delta_t(&ndt.date()) / 86400_f64
}

#[cfg(test)]
mod tests {
    use chrono::prelude::*;
    use time::nasa::*;

    fn within(a : f64, b : f64, epsilon : f64) -> bool {
        (a - b).abs() <= epsilon
    }

    #[test]
    fn delta_t_tests() {
        struct CheckSet { date : chrono::NaiveDate, expected : f64, within : f64 };

        // test data from https://eclipse.gsfc.nasa.gov/SEcat5/deltat.html
        // (tables 1 and 2)
        let sets = [
            // Table 1:
            //
            // For this item, math seems to check out, but significantly higher variance...
            CheckSet { date: chrono::NaiveDate::from_ymd(-500, 01, 01), expected: 17190_f64, within : 13_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(-400, 01, 01), expected: 15530_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(-300, 01, 01), expected: 14080_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(-200, 01, 01), expected: 12790_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(-100, 01, 01), expected: 11640_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(0000, 01, 01), expected: 10580_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(0100, 01, 01), expected: 09600_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(0200, 01, 01), expected: 08640_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(0300, 01, 01), expected: 07680_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(0400, 01, 01), expected: 06700_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(0500, 01, 01), expected: 05710_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(0600, 01, 01), expected: 04740_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(0700, 01, 01), expected: 03810_f64, within : 4_f64 },
            // For this item, math seems to check out, slightly higher variance?
            CheckSet { date: chrono::NaiveDate::from_ymd(0800, 01, 01), expected: 02960_f64, within : 5_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(0900, 01, 01), expected: 02200_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1000, 01, 01), expected: 01570_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1100, 01, 01), expected: 01090_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1200, 01, 01), expected: 00740_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1300, 01, 01), expected: 00490_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1400, 01, 01), expected: 00320_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1500, 01, 01), expected: 00200_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1600, 01, 01), expected: 00120_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1700, 01, 01), expected: 00009_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1750, 01, 01), expected: 00013_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1800, 01, 01), expected: 00014_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1850, 01, 01), expected: 00007_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1900, 01, 01), expected: -0003_f64, within : 4_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1950, 01, 01), expected: 00029_f64, within : 4_f64 },

            // Table 2:
            CheckSet { date: chrono::NaiveDate::from_ymd(1955, 01, 01), expected: 31.1, within : 1_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1960, 01, 01), expected: 33.2, within : 1_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1965, 01, 01), expected: 35.7, within : 1_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1970, 01, 01), expected: 40.2, within : 1_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1975, 01, 01), expected: 45.5, within : 1_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1980, 01, 01), expected: 50.5, within : 1_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1985, 01, 01), expected: 54.3, within : 1_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1990, 01, 01), expected: 56.9, within : 1_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(1995, 01, 01), expected: 60.8, within : 1_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(2000, 01, 01), expected: 63.8, within : 1_f64 },
            CheckSet { date: chrono::NaiveDate::from_ymd(2005, 01, 01), expected: 64.7, within : 1_f64 },
        ];

        for set in sets.iter() {
            let val = delta_t(&set.date);
            assert!(
                within(val, set.expected, set.within),
                format!("Expected NASA ΔT to be within {} seconds (date: {}, expected: {}, actual: {})",
                    set.within,
                    set.date,
                    set.expected,
                    val));
        }
    }
}