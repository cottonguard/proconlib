use crate::dsu::*;

#[test]
fn dsu_merge() {
    let mut dsu = DsuMerge::from_fn(10, |i| format!("{i}"), |x, y| x.push_str(&y));

    let (res, data) = dsu.unite(1, 2);
    assert!(res.is_united());
    assert_eq!(res.root, 1);
    assert_eq!(res.united_root, Some(2));
    assert_eq!(res.size, 2);
    assert!(&*data == "12");

    let (res, data) = dsu.unite(1, 2);
    assert!(!res.is_united());
    assert_eq!(res.root, 1);
    assert_eq!(res.united_root, None);
    assert_eq!(res.size, 2);
    assert!(&*data == "12");

    assert_eq!(dsu.root(1), 1);
    assert_eq!(dsu.root(2), 1);
    assert!(dsu.is_same(1, 2));
    assert!(dsu.is_same(1, 1));
    assert!(!dsu.is_same(1, 3));
    assert_eq!(dsu.data(1), "12");
    assert_eq!(dsu.data(2), "12");
}
