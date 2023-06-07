use crate::input::*;

#[test]
fn sprit_ws() {
    let s = b"a  bc\n \ndef";
    let mut sw = SplitWs::new(&s[..]);
    assert_eq!(sw.bytes(), b"a");
    assert_eq!(sw.bytes(), b"bc");
    assert_eq!(sw.bytes(), b"def");
    assert_eq!(sw.bytes(), b"");
}

#[test]
fn uint() {
    let s = b"0 1 12345678 255";
    let mut sw = SplitWs::new(&s[..]);
    assert_eq!(sw.parse::<u32>(), 0);
    assert_eq!(sw.parse::<u32>(), 1);
    assert_eq!(sw.parse::<u32>(), 12345678);
    assert_eq!(sw.parse::<u8>(), 255);
}

#[test]
fn int() {
    let s = b"0 1 123 -123 127 -128";
    let mut sw = SplitWs::new(&s[..]);
    assert_eq!(sw.parse::<i32>(), 0);
    assert_eq!(sw.parse::<i32>(), 1);
    assert_eq!(sw.parse::<i32>(), 123);
    assert_eq!(sw.parse::<i32>(), -123);
    assert_eq!(sw.parse::<i8>(), 127);
    assert_eq!(sw.parse::<i8>(), -128);
}

#[test]
fn float() {
    let s = b"0 1 0.5 -30 -30.0 1000000000000 0.00000000001";
    let mut sw = SplitWs::new(&s[..]);
    assert_eq!(sw.parse::<f64>(), 0.0);
    assert_eq!(sw.parse::<f64>(), 1.0);
    assert_eq!(sw.parse::<f64>(), 0.5);
    assert_eq!(sw.parse::<f64>(), -30.0);
    assert_eq!(sw.parse::<f64>(), -30.0);
    assert_eq!(sw.parse::<f64>(), 1000000000000.0);
    assert_eq!(sw.parse::<f64>(), 0.00000000001);
}

#[test]
fn owned() {
    let s = b"Vec Boxed-slice String";
    let mut sw = SplitWs::new(&s[..]);
    assert_eq!(sw.parse::<Vec<u8>>(), b"Vec");
    assert_eq!(&sw.parse::<Box<[u8]>>()[..], b"Boxed-slice");
    assert_eq!(sw.parse::<String>(), "String");
}

#[test]
fn tuple() {
    let s = b"1 a 2 b 3 c";
    let mut sw = SplitWs::new(&s[..]);
    assert_eq!(sw.parse::<(i32, char)>(), (1, 'a'));
    assert_eq!(sw.parse::<(i32, char)>(), (2, 'b'));
    assert_eq!(sw.parse::<(i32, char)>(), (3, 'c'));
}

#[test]
fn seq() {
    let s = b"1 2 3 4 5";
    let mut sw = SplitWs::new(&s[..]);
    assert_eq!(sw.seq::<i32>().take(5).collect::<Vec<_>>(), [1, 2, 3, 4, 5]);
}

#[test]
fn per_n_bytes() {
    struct PerNBytes<'a> {
        chunks: std::slice::Chunks<'a, u8>,
    }
    impl<'a> PerNBytes<'a> {
        fn new(s: &'a [u8], n: usize) -> Self {
            Self {
                chunks: s.chunks(n),
            }
        }
    }
    impl<'a> std::io::Read for PerNBytes<'a> {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let s = self.chunks.next().unwrap_or(&[]);
            buf[..s.len()].copy_from_slice(s);
            Ok(s.len())
        }
    }

    let s = b"quick brown fox jumps over the lazy dog";
    for n in 1..=8 {
        let p = PerNBytes::new(s, n);
        let mut sw = SplitWs::new(p);
        assert_eq!(sw.bytes(), b"quick");
        assert_eq!(sw.bytes(), b"brown");
        assert_eq!(sw.bytes(), b"fox");
        assert_eq!(sw.bytes(), b"jumps");
        assert_eq!(sw.bytes(), b"over");
        assert_eq!(sw.bytes(), b"the");
        assert_eq!(sw.bytes(), b"lazy");
        assert_eq!(sw.bytes(), b"dog");
        assert_eq!(sw.bytes(), b"");
        assert_eq!(sw.bytes(), b"");
    }
}
