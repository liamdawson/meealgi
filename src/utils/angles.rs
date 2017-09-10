const TWO_PI : f64 = ::std::f64::consts::PI * 2_f64;

pub fn limit_radians(rad : f64) -> f64 {
    let revs = rad / TWO_PI;
    let limited = TWO_PI * (revs - revs.floor());

    limited
}