extern "C" {
    pub fn isatty(fd: i32) -> i32;
}

#[macro_export]
macro_rules! w {
    ($dst:expr, $($arg:tt)*) => {
        if cfg!(debug_assertions) && unsafe { $crate::macros::isatty(1) } != 0 {
            write!($dst, "\x1B[1;33m").unwrap();
            write!($dst, $($arg)*).unwrap();
            write!($dst, "\x1B[0m").unwrap();
        } else {
            write!($dst, $($arg)*).unwrap();
        }
    }
}
#[macro_export]
macro_rules! wln {
    ($dst:expr $(, $($arg:tt)*)?) => {{
        if cfg!(debug_assertions) && unsafe { $crate::macros::isatty(1) } != 0 {
            write!($dst, "\x1B[1;33m").unwrap();
            writeln!($dst $(, $($arg)*)?).unwrap();
            write!($dst, "\x1B[0m").unwrap();
        } else {
            writeln!($dst $(, $($arg)*)?).unwrap();
        }
        #[cfg(debug_assertions)]
        $dst.flush().unwrap();
    }}
}
#[macro_export]
macro_rules! w_iter {
    ($dst:expr, $fmt:expr, $iter:expr, $delim:expr) => {{
        let mut first = true;
        for elem in $iter {
            if first {
                w!($dst, $fmt, elem);
                first = false;
            } else {
                w!($dst, concat!($delim, $fmt), elem);
            }
        }
    }};
    ($dst:expr, $fmt:expr, $iter:expr) => {
        w_iter!($dst, $fmt, $iter, " ")
    };
}
#[macro_export]
macro_rules! w_iter_ln {
    ($dst:expr, $($t:tt)*) => {{
        w_iter!($dst, $($t)*);
        wln!($dst);
    }}
}
#[macro_export]
macro_rules! e {
    ($($t:tt)*) => {
        #[cfg(debug_assertions)]
        eprint!($($t)*)
    }
}
#[macro_export]
macro_rules! eln {
    ($($t:tt)*) => {
        #[cfg(debug_assertions)]
        eprintln!($($t)*)
    }
}
#[macro_export]
#[doc(hidden)]
macro_rules! __tstr {
    ($h:expr $(, $t:expr)+) => { concat!(__tstr!($($t),+), ", ", __tstr!(@)) };
    ($h:expr) => { concat!(__tstr!(), " ",  __tstr!(@)) };
    () => { "\x1B[94m[{}:{}]\x1B[0m" };
    (@) => { "\x1B[1;92m{}\x1B[0m = {:?}" }
}
#[macro_export]
macro_rules! d {
    ($($a:expr),*) => {
        if std::env::var("ND").map(|v| &v == "0").unwrap_or(true) {
            eln!(__tstr!($($a),*), file!(), line!(), $(stringify!($a), $a),*);
        }
    };
}
