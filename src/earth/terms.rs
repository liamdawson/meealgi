use earth::periodic_terms;
use utils::angles::limit_radians;

/// Calculates the heliocentric longitude, in radians
///
/// Obtained from NREL SPA report
/// TODO: examples
pub fn heliocentric_longitude(jul_mil_ephemeris: f64) -> f64 {
    let mut ls: Vec<Vec<periodic_terms::EarthPeriodicTableRow>> =
        vec![vec![], vec![], vec![], vec![], vec![], vec![]];

    for row in periodic_terms::EARTH_PERIODIC_TERMS {
        match row.term {
            "L0" => ls[0].push(*row),
            "L1" => ls[1].push(*row),
            "L2" => ls[2].push(*row),
            "L3" => ls[3].push(*row),
            "L4" => ls[4].push(*row),
            "L5" => ls[5].push(*row),
            _ => {}
        }
    }

    let l_vals = (0..6)
        .map(|i| {
            ls[i]
                .iter()
                .map(|row| row.a * (row.b + row.c * jul_mil_ephemeris).cos())
                .sum::<f64>()
        })
        .collect::<Vec<f64>>();

    println!(
        "{:?}",
        [
            l_vals[0],
            l_vals[1],
            l_vals[2],
            l_vals[3],
            l_vals[4],
            l_vals[5],
        ]
    );

    let result =
        (l_vals[0] +
             jul_mil_ephemeris *
                 (l_vals[1] +
                      jul_mil_ephemeris *
                          (l_vals[2] +
                               jul_mil_ephemeris *
                                   (l_vals[3] +
                                        jul_mil_ephemeris *
                                            (l_vals[4] + jul_mil_ephemeris * l_vals[5]))))) /
            100_000_000_f64;

    // limit to a single rotation
    // TODO: test for negatives
    // result % (::std::f64::consts::PI * 2_f64)
    limit_radians(result)
}

/// Calculates the heliocentric latitude, in radians
///
/// Obtained from NREL SPA report
/// TODO: examples
pub fn heliocentric_latitude(jul_mil_ephemeris: f64) -> f64 {
    let b0s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| {
        row.term == "B0"
    });
    let b1s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| {
        row.term == "B1"
    });

    let b0 = b0s.fold(0_f64, |curr, row| {
        curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos()
    });
    let b1 = b1s.fold(0_f64, |curr, row| {
        curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos()
    });

    let result = (b0 + b1 * jul_mil_ephemeris) / 10_f64.powi(8);

    // limit to a single rotation
    // TODO: test for negatives
    result % (::std::f64::consts::PI * 2_f64)
}

/// Calculates the earth radius vector, in Astronomical Units
///
/// Obtained from NREL SPA report
/// TODO: examples
pub fn radius_vec(jul_mil_ephemeris: f64) -> f64 {
    let r0s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| {
        row.term == "R0"
    });
    let r1s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| {
        row.term == "R1"
    });
    let r2s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| {
        row.term == "R2"
    });
    let r3s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| {
        row.term == "R3"
    });
    let r4s = periodic_terms::EARTH_PERIODIC_TERMS.iter().filter(|row| {
        row.term == "R4"
    });

    let r0 = r0s.fold(0_f64, |curr, row| {
        curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos()
    });
    let r1 = r1s.fold(0_f64, |curr, row| {
        curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos()
    });
    let r2 = r2s.fold(0_f64, |curr, row| {
        curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos()
    });
    let r3 = r3s.fold(0_f64, |curr, row| {
        curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos()
    });
    let r4 = r4s.fold(0_f64, |curr, row| {
        curr + row.a * (row.b + row.c * jul_mil_ephemeris).cos()
    });

    let result = (r0 + r1 * jul_mil_ephemeris + r2 * jul_mil_ephemeris.powi(2) +
                      r3 * jul_mil_ephemeris.powi(3) +
                      r4 * jul_mil_ephemeris.powi(4)) / 10_f64.powi(8);

    // limit to a single rotation
    // TODO: test for negatives
    result % (::std::f64::consts::PI * 2_f64)
}
