pub trait Bounded {
    fn min_value() -> Self;
    fn max_value() -> Self;
}

/*
macro_rules! bounded {
    ($ty:ty) => {
        impl Bounded for $ty {
            fn min_value() -> Self {
                <$ty>::min_value()
            }
            fn max_value() -> Self {
                <$ty>::min_value()
            }
        }
    };
}

macro_rules! impls {
    ($($ty:ty),*) => {$(bounded!($ty);)*};
}

impls!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);
*/
