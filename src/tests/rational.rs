use crate::random::*;
use crate::rational::*;

#[test]
fn test() {
    assert_eq!(Rational::new(5, 10), Rational::unnormalized(1, 2));
    assert_eq!(Rational::new(0, 100), Rational::unnormalized(0, 1));
    assert_eq!(
        Rational::new(1, 2) + Rational::new(1, 3),
        Rational::unnormalized(5, 6)
    );
    assert_eq!(
        Rational::new(1, 3) + Rational::new(2, 3),
        Rational::unnormalized(1, 1)
    );
    assert_eq!(
        Rational::new(1, 3) + Rational::new(-2, 3),
        Rational::unnormalized(-1, 3)
    );
    assert_eq!(
        Rational::new(-1, 3) + Rational::new(-2, 3),
        Rational::unnormalized(-1, 1)
    );
    assert_eq!(
        Rational::new((1 << 62) + 1, 2) + Rational::new((1 << 62) - 1, 2),
        Rational::new(1 << 62, 1)
    );
    assert_eq!(
        Rational::new((3 << 61) + 1, 3) + Rational::new((3 << 61) - 1, 3),
        Rational::new(1 << 62, 1)
    );
    assert_eq!(
        Rational::new(-((1 << 62) + 1), 2) + Rational::new(-((1 << 62) - 1), 2),
        Rational::new(-(1 << 62), 1)
    );
    assert_eq!(
        Rational::new(-((3 << 61) + 1), 3) + Rational::new(-((3 << 61) - 1), 3),
        Rational::new(-(1 << 62), 1)
    );
    assert_eq!(
        Rational::new(1, 2) - Rational::new(1, 3),
        Rational::unnormalized(1, 6)
    );
    assert_eq!(
        Rational::new(1, 3) - Rational::new(1, 3),
        Rational::unnormalized(0, 1)
    );
    assert_eq!(
        Rational::new(1, 2) * Rational::new(3, 4),
        Rational::unnormalized(3, 8)
    );
    assert_eq!(
        Rational::new(2, 3) * Rational::new(3, 4),
        Rational::unnormalized(1, 2)
    );
    assert_eq!(
        Rational::new(1, 2) / Rational::new(3, 4),
        Rational::unnormalized(2, 3)
    );
}

#[test]
fn ratio_int() {
    assert_eq!(Rational::new(1, 2) + 2, Rational::new(5, 2));
    assert_eq!(Rational::new(1, 2) - 2, Rational::new(-3, 2));
    assert_eq!(Rational::new(1, 2) * 2, Rational::new(1, 1));
    assert_eq!(Rational::new(1, 2) / 2, Rational::new(1, 4));
    assert_eq!(Rational::new(2, 3) + 2, Rational::new(8, 3));
    assert_eq!(Rational::new(2, 3) - 2, Rational::new(-4, 3));
    assert_eq!(Rational::new(2, 3) * 2, Rational::new(4, 3));
    assert_eq!(Rational::new(2, 3) / 2, Rational::new(1, 3));
}

#[test]
fn int_fract_random() {
    let mut rng = Xoshiro::seed_from_u64(1);
    for _ in 0..100 {
        let num = rng.range(0, 1 << 60);
        let den = rng.range(1, 1 << 40);
        let ratio = Rational::new(num, den);
        let (int, fract) = ratio.int_fract();
        assert_eq!(int + fract, ratio);
    }
}

#[test]
fn cmp() {
    assert!(Rational::new(1, 2) > Rational::new(1, 3));
    assert!(Rational::new(1, 2) > Rational::new(0, 1));
    assert!(Rational::new(5, 3) > Rational::new(4, 3));
}
