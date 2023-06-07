use crate::int::*;

#[test]
fn gcd() {
    assert_eq!(100.gcd(100), 100);
    assert_eq!(1.gcd(100), 1);
    assert_eq!(6.gcd(9), 3);
    assert_eq!(9.gcd(6), 3);
    assert_eq!(10.gcd(100), 10);
    assert_eq!(100.gcd(10), 10);
    assert_eq!((-100).gcd(100), 100);
    assert_eq!((-100).gcd(-100), 100);
    assert_eq!(0.gcd(100), 100);
    assert_eq!(100.gcd(0), 100);
    assert_eq!(0.gcd(0), 0);
}

#[test]
fn div_floor() {
    #![allow(unstable_name_collisions)]

    assert_eq!(<i32 as UInt>::div_floor(0, 1), 0);
    assert_eq!(<i32 as UInt>::div_floor(0, -1), 0);
    assert_eq!(<i32 as UInt>::div_floor(1, 1), 1);
    assert_eq!(<i32 as UInt>::div_floor(3, 2), 1);
    assert_eq!(<i32 as UInt>::div_floor(4, 2), 2);
    assert_eq!(<i32 as UInt>::div_floor(-1, 1), -1);
    assert_eq!(<i32 as UInt>::div_floor(-3, 2), -2);
    assert_eq!(<i32 as UInt>::div_floor(-4, 2), -2);
    assert_eq!(<i32 as UInt>::div_floor(1, -1), -1);
    assert_eq!(<i32 as UInt>::div_floor(3, -2), -2);
    assert_eq!(<i32 as UInt>::div_floor(4, -2), -2);
    assert_eq!(<i32 as UInt>::div_floor(-1, -1), 1);
    assert_eq!(<i32 as UInt>::div_floor(-3, -2), 1);
    assert_eq!(<i32 as UInt>::div_floor(-4, -2), 2);
}

#[test]
fn div_ceil() {
    #![allow(unstable_name_collisions)]

    assert_eq!(<i32 as UInt>::div_ceil(0, 1), 0);
    assert_eq!(<i32 as UInt>::div_ceil(0, -1), 0);
    assert_eq!(<i32 as UInt>::div_ceil(1, 1), 1);
    assert_eq!(<i32 as UInt>::div_ceil(3, 2), 2);
    assert_eq!(<i32 as UInt>::div_ceil(4, 2), 2);
    assert_eq!(<i32 as UInt>::div_ceil(-1, 1), -1);
    assert_eq!(<i32 as UInt>::div_ceil(-3, 2), -1);
    assert_eq!(<i32 as UInt>::div_ceil(-4, 2), -2);
    assert_eq!(<i32 as UInt>::div_ceil(1, -1), -1);
    assert_eq!(<i32 as UInt>::div_ceil(3, -2), -1);
    assert_eq!(<i32 as UInt>::div_ceil(4, -2), -2);
    assert_eq!(<i32 as UInt>::div_ceil(-1, -1), 1);
    assert_eq!(<i32 as UInt>::div_ceil(-3, -2), 2);
    assert_eq!(<i32 as UInt>::div_ceil(-4, -2), 2);
}
