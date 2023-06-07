use crate::f2::*;

#[test]
fn test() {
    let mut a = F2::zero(200);
    assert_eq!(a.get(0), false);
    assert_eq!(a.get(10), false);
    assert_eq!(a.get(199), false);
    assert_eq!(a.set(0, true), false);
    assert_eq!(a.set(0, true), true);
    assert_eq!(a.set(199, true), false);

    let cloned = a.clone();
    assert_eq!(a, cloned);

    assert_eq!(!!a.clone(), a);
}
