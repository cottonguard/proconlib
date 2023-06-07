use crate::random::*;
use crate::rbstree::*;

#[test]
fn insert_remove() {
    let mut t = RbsTree::new();
    assert_eq!(t.get(&1), None);
    assert_eq!(t.insert(1, 10), None);
    assert_eq!(t.get(&1), Some(&10));
    assert_eq!(t.get(&2), None);
    assert_eq!(t.insert(2, 20), None);
    assert_eq!(t.get(&2), Some(&20));
    assert_eq!(t.insert(1, 11), Some(10));
    assert_eq!(t.get(&1), Some(&11));
}

#[test]
fn insert_remove_random() {
    let mut rng = Xoshiro::seed_from_u64(1);
    let mut t = RbsTree::new();
    let mut map = std::collections::BTreeMap::new();
    for _ in 0..100 {
        let k = rng.range(0, 10);
        let v: i32 = rng.gen();
        t.insert(k, v);
        map.insert(k, v);
        let k = rng.range(0, 10);
        assert_eq!(t.get(&k), map.get(&k));
        dump_keys(&t, std::io::stderr().lock());
    }
}
