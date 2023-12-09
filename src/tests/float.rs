use crate::float::*;

#[test]
fn bisect_test() {
    assert_eq!(bisect(0.0, 10.0, |x| x >= 1.0), 1.0);
    assert_eq!(bisect(-10.0, 10.0, |x| x >= 1.0), 1.0);
    assert_eq!(bisect(0.0, 10.0, |x| x >= 0.0), 0.0);
    assert_eq!(bisect(-10.0, 10.0, |x| x >= 0.0), 0.0);
    assert_eq!(bisect(-10.0, 0.0, |x| x >= -1.0), -1.0);
    assert_eq!(bisect(-10.0, 10.0, |x| x >= -1.0), -1.0);
    assert!(bisect(-10.0, f64::NAN, |x| x >= 1.0).is_nan());
    assert_eq!(bisect(0.0, f64::INFINITY, |x| x >= 1.0), 1.0);
    assert_eq!(bisect(-f64::INFINITY, f64::INFINITY, |x| x >= 1.0), 1.0);
}
