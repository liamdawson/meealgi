use ::earth::periodic_terms;

/// Calculates the heliocentric longitude, in radians
/// TODO: examples
pub fn heliocentric_longitude(jul_mil_ephemeris : f64) -> f64 {
    let l0s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| row.term == "L0");
    let l1s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| row.term == "L1");
    let l2s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| row.term == "L2");
    let l3s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| row.term == "L3");
    let l4s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| row.term == "L4");
    let l5s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| row.term == "L5");

    let l0 = l0s.fold(0_f64, |curr, row| curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos());
    let l1 = l1s.fold(0_f64, |curr, row| curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos());
    let l2 = l2s.fold(0_f64, |curr, row| curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos());
    let l3 = l3s.fold(0_f64, |curr, row| curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos());
    let l4 = l4s.fold(0_f64, |curr, row| curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos());
    let l5 = l5s.fold(0_f64, |curr, row| curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos());

    let result = (l0 + l1 * jul_mil_ephemeris
        + l2 * jul_mil_ephemeris.powi(2)
        + l3 * jul_mil_ephemeris.powi(3)
        + l4 * jul_mil_ephemeris.powi(4)
        + l5 * jul_mil_ephemeris.powi(5)
        ) / 10_f64.powi(8);

    // limit to a single rotation
    // TODO: test for negatives
    result % (::std::f64::consts::PI * 2_f64)
}

/// Calculates the heliocentric latitude, in radians
/// TODO: examples
pub fn heliocentric_latitude(jul_mil_ephemeris : f64) -> f64 {
    let b0s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| row.term == "B0");
    let b1s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| row.term == "B1");

    let b0 = b0s.fold(0_f64, |curr, row| curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos());
    let b1 = b1s.fold(0_f64, |curr, row| curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos());

    let result = (b0 + b1 * jul_mil_ephemeris) / 10_f64.powi(8);

    // limit to a single rotation
    // TODO: test for negatives
    result % (::std::f64::consts::PI * 2_f64)
}

/// Calculates the earth radius vector, in Astronomical Units
/// TODO: examples
pub fn earth_radius_vec(jul_mil_ephemeris : f64) -> f64 {
    let r0s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| row.term == "R0");
    let r1s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| row.term == "R1");
    let r2s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| row.term == "R2");
    let r3s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| row.term == "R3");
    let r4s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| row.term == "R4");

    let r0 = r0s.fold(0_f64, |curr, row| curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos());
    let r1 = r1s.fold(0_f64, |curr, row| curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos());
    let r2 = r2s.fold(0_f64, |curr, row| curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos());
    let r3 = r3s.fold(0_f64, |curr, row| curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos());
    let r4 = r4s.fold(0_f64, |curr, row| curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos());

    let result = (r0 + r1 * jul_mil_ephemeris
        + r2 * jul_mil_ephemeris.powi(2)
        + r3 * jul_mil_ephemeris.powi(3)
        + r4 * jul_mil_ephemeris.powi(4)
        ) / 10_f64.powi(8);

    // limit to a single rotation
    // TODO: test for negatives
    result % (::std::f64::consts::PI * 2_f64)
}
