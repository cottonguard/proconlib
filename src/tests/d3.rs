use crate::d3::*;

macro_rules! assert_abs_diff_eq {
    ($x:expr, $y:expr) => {
        assert!($x.abs_diff_eq($y, 1e-9), "left={:?}, right={:?}", $x, $y)
    };
}

#[test]
fn mat_float() {
    let a = mat3(
        vec3(3.0, 1.0, 4.0),
        vec3(1.0, 5.0, 9.0),
        vec3(2.0, 6.0, 5.0),
    );
    assert_abs_diff_eq!(a * a.inv(), Mat3::<f64>::identity());
}
