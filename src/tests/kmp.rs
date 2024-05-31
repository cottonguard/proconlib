use crate::kmp::*;

#[test]
fn test() {
    fn case<T: Eq>(pat: &[T], s: &[T], expected: &[usize]) {
        let kmp = Kmp::new(pat);
        let res: Vec<usize> = kmp.positions(s).collect();
        assert_eq!(res, expected);
    }

    case(b"a", b"abcabcabcxabcab", &[0, 3, 6, 10, 13]);
    case(b"abcab", b"abcabcabcxabcab", &[0, 3, 10]);
    case(b"xxx", b"xxxxxxxxxx", &[0, 1, 2, 3, 4, 5, 6, 7]);
}
