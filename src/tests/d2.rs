use crate::d2::*;

macro_rules! assert_abs_diff_eq {
    ($x:expr, $y:expr) => {
        assert!($x.abs_diff_eq($y, 1e-9), "left={:?}, right={:?}", $x, $y);
    };
}

#[test]
fn vec2_int() {
    assert_eq!(vec2(1, 2) + vec2(3, 4), vec2(4, 6));
    assert_eq!(vec2(1, 2) - vec2(3, 4), vec2(-2, -2));
    assert_eq!(vec2(1, 2) * vec2(3, 4), vec2(3, 8));
    assert_eq!(vec2(1, 2) * 3, vec2(3, 6));
    assert_eq!(vec2(4, 6) / vec2(2, 3), vec2(2, 2));
    assert_eq!(vec2(4, 6) / 2, vec2(2, 3));
    assert_eq!(-vec2(1, 2), vec2(-1, -2));
    assert_eq!(vec2(1, 2).dot(vec2(3, 4)), 1 * 3 + 2 * 4);
    assert_eq!(vec2(1, 2).cross(vec2(3, 4)), 1 * 4 - 2 * 3);
    assert_eq!(vec2(3, 4).norm(), 3 * 3 + 4 * 4);
}

#[test]
fn vec2_float() {
    assert_abs_diff_eq!(vec2(1.0, 2.0) + vec2(3.0, 4.0), vec2(4.0, 6.0));
    assert_abs_diff_eq!(vec2(3.0, 4.0).length(), 5.0);
    assert_abs_diff_eq!(
        vec2(1.0, f64::sqrt(3.0)).angle(),
        std::f64::consts::PI / 3.0
    );
    assert_abs_diff_eq!(
        vec2(1.0, -f64::sqrt(3.0)).angle(),
        -std::f64::consts::PI / 3.0
    );
    assert_abs_diff_eq!(vec2(1.0, 2.0).lerp(vec2(3.0, 4.0), 0.2), vec2(1.4, 2.4));
    assert_abs_diff_eq!(
        vec2(1.0, 1.0).rotate_angle(std::f64::consts::FRAC_PI_4),
        vec2(0.0, 2.0f64.sqrt())
    );
    assert_abs_diff_eq!(vec2(1.0, 1.0).rotate(vec2(1.0, 1.0)), vec2(0.0, 2.0));
    assert_abs_diff_eq!(
        vec2(10.0, 10.0).normalize(),
        vec2(
            1.0 / std::f64::consts::SQRT_2,
            1.0 / std::f64::consts::SQRT_2
        )
    );
    assert_abs_diff_eq!(vec2(2.0, 2.0).project_onto(vec2(3.0, 0.0)), vec2(2.0, 0.0));
}

#[test]
fn vec2_conversion() {
    assert_eq!(vec2(10i32, 20).cast::<i64>(), vec2(10i64, 20));
    assert_eq!(<[i32; 2]>::from(vec2(1, 2)), [1, 2]);
    assert_eq!(<(i32, i32)>::from(vec2(1, 2)), (1, 2));
    assert_eq!(<Vec2<i32>>::from([1, 2]), vec2(1, 2));
    assert_eq!(<Vec2<i32>>::from((1, 2)), vec2(1, 2));
}

#[test]
fn mat2_int() {
    assert_eq!(Mat2::diag(vec2(1, 2)), Mat2::from_elems(1, 0, 0, 2));
    assert_eq!(
        Mat2::from_elems(1, 2, 3, 4).transpose(),
        Mat2::from_elems(1, 3, 2, 4)
    );
}

#[test]
fn mat2_float() {
    let m = Mat2::from_elems(1.0, 2.0, 3.0, 4.0);
    assert_abs_diff_eq!(m * m.inv(), Mat2::<f64>::identity());
}

#[test]
fn mat2_vec2() {
    assert_eq!(mat2(vec2(1, 2), vec2(3, 4)) * vec2(5, 6), vec2(17, 39));
    assert_eq!(Mat2::identity() * vec2(5, 6), vec2(5, 6));
    assert_abs_diff_eq!(
        Mat2::scale_rotation(vec2(2.0, 3.0), std::f64::consts::FRAC_PI_3)
            * vec2(1.0, 3.0f64.sqrt()),
        vec2(-2.0, 3.0 * 3.0f64.sqrt())
    );
}

#[test]
fn affine2_float() {
    let a = Affine2::new(Mat2::from_elems(1.0, 2.0, 3.0, 4.0), vec2(5.0, 6.0));
    assert_abs_diff_eq!(a * a.inv(), Affine2::<f64>::identity());
}

#[test]
fn affine2_vec2() {}
