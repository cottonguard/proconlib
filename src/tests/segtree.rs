/*

use crate::segtree::*;

#[test]
fn max_right() {
    impl Monoid for i32 {
        fn id() -> Self {
            0
        }
        fn op(&self, other: &Self) -> Self {
            self + other
        }
    }

    let st = SegTree::from(&[10; 5][..]);
    assert_eq!(st.max_right(0, |x| *x <= 30), (3, 30));
    assert_eq!(st.max_right(1, |x| *x <= 30), (4, 30));
    assert_eq!(st.max_right(2, |x| *x <= 30), (5, 30));
    assert_eq!(st.max_right(3, |x| *x <= 30), (5, 20));
}
 */
