extern crate meealgi;
extern crate chrono;

fn get_jme(time : &chrono::NaiveDateTime) -> f64 {
    let jd = meealgi::time::ndt_to_jul(&time);
    println!("{}", jd);
    let jde = meealgi::time::nasa::ndt_to_jule(&time);
    println!("{}", jde);
    let jce = meealgi::time::jul_to_julc(jde);
    println!("{}", jce);
    meealgi::time::julc_to_julm(jce)
}

fn heliocentric_earth_from_time(time : &chrono::NaiveDateTime) -> [f64; 3] {
    let jme = get_jme(&time);
    [
        meealgi::earth::heliocentric_latitude(jme).to_degrees(),
        meealgi::earth::heliocentric_longitude(jme).to_degrees(),
        meealgi::earth::radius_vec(jme)
    ]
}

fn round_f64(val : f64, places : i32) -> f64 {
    (val * 10f64.powi(places)).round() / 10f64.powi(places)
}

#[test]
// TODO: determine whether there's a bug causing the discrepencies
#[ignore]
fn calculates_expected_position() {
    let test_date = chrono::NaiveDate::from_ymd(2017, 08, 30).and_hms(12, 0, 0);
    let expected : Vec<f64> = vec![-0.000168_f64, 337.315687_f64, 1.009582_f64];
    let actual : Vec<f64> = heliocentric_earth_from_time(&test_date)
        .into_iter()
        .map(|v| round_f64(*v, 6))
        .collect();

    assert_eq!(expected, actual);
}