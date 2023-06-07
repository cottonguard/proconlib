use crate::bits::*;

#[test]
fn gray_code_test() {
    for n in 0..=6 {
        let mut g: Vec<_> = gray_code::<u32>(n).collect();
        assert!(g.windows(2).all(|h| (h[0] ^ h[1]).count_ones() == 1));
        g.sort();
        assert!(g.into_iter().eq(0..1 << n));
    }
}
