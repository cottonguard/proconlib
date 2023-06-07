use crate::light_vec;
use crate::light_vec::*;

#[test]
fn push() {
    let mut v = LightVec::<i32, 0>::new();
    for i in 0..10 {
        v.push(i);
    }

    let mut v = LightVec::<i32, 5>::new();
    for i in 0..10 {
        v.push(i);
    }

    let mut v = LightVec::<i32, 20>::new();
    for i in 0..10 {
        v.push(i);
    }

    let mut v = LightVec::<String, 5>::new();
    for i in 0..10 {
        v.push(format!("{i}"));
    }

    let mut v = LightVec::<String, 20>::new();
    for i in 0..10 {
        v.push(format!("{i}"));
    }
}

#[test]
fn clone() {
    let mut v = LightVec::<String, 5>::new();
    for i in 0..10 {
        v.push(format!("{i}"));
    }
    let cloned = v.clone();
    assert_eq!(v, cloned);

    let mut v = LightVec::<String, 20>::new();
    for i in 0..10 {
        v.push(format!("{i}"));
    }
    let cloned = v.clone();
    assert_eq!(v, cloned);
}

#[test]
fn from_vec() {
    let v = vec![];
    let l = LightVec::<i32, 5>::from(v.clone());
    assert_eq!(v.as_slice(), l.as_slice());

    let v = vec![1, 2, 3];
    let l = LightVec::<i32, 5>::from(v.clone());
    assert_eq!(v.as_slice(), l.as_slice());

    let v = vec![1, 2, 3];
    let l = LightVec::<i32, 2>::from(v.clone());
    assert_eq!(v.as_slice(), l.as_slice());
}

#[test]
fn insert() {
    let mut v = LightVec::<i32, 5>::from(vec![1, 2, 3]);
    assert_eq!(*v, [1, 2, 3][..]);
    v.insert(0, 4);
    assert_eq!(*v, [4, 1, 2, 3][..]);
    v.insert(2, 5);
    assert_eq!(*v, [4, 1, 5, 2, 3][..]);
    v.insert(2, 6);
    assert_eq!(*v, [4, 1, 6, 5, 2, 3][..]);
    v.insert(6, 7);
    assert_eq!(*v, [4, 1, 6, 5, 2, 3, 7][..]);
}

#[test]
#[should_panic]
fn insert_oor() {
    let mut v = LightVec::<i32, 5>::from(vec![1, 2, 3]);
    v.insert(4, 4);
}

#[test]
fn remove() {
    let mut v = LightVec::<i32, 5>::from(vec![1, 2, 3, 4]);
    assert_eq!(v.remove(1), 2);
    assert_eq!(v[..], [1, 3, 4]);
    assert_eq!(v.remove(2), 4);
    assert_eq!(v[..], [1, 3]);
    assert_eq!(v.remove(0), 1);
    assert_eq!(v[..], [3]);
}

#[test]
fn into_iter() {
    let v = LightVec::<i32, 5>::from(vec![1, 2, 3]);
    let mut iter = v.into_iter();
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.next_back(), Some(3));
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.next_back(), None);

    let v = LightVec::<i32, 0>::from(vec![1, 2, 3]);
    let mut iter = v.into_iter();
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.next_back(), Some(3));
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.next_back(), None);

    let v = LightVec::<String, 5>::from(vec!["a".into(), "b".into(), "c".into(), "d".into()]);
    let mut iter = v.into_iter();
    assert_eq!(iter.next().as_deref(), Some("a"));
    assert_eq!(iter.next_back().as_deref(), Some("d"));
}

#[test]
fn into_iter_nth() {
    let v = LightVec::<String, 5>::from(vec!["a".into(), "b".into(), "c".into(), "d".into()]);
    let mut iter = v.into_iter();
    assert_eq!(iter.nth(0).as_deref(), Some("a"));
    assert_eq!(iter.nth(1).as_deref(), Some("c"));
    assert_eq!(iter.nth(100), None);

    let v = LightVec::<String, 5>::from(vec!["a".into(), "b".into(), "c".into(), "d".into()]);
    let mut iter = v.into_iter();
    assert_eq!(iter.nth_back(0).as_deref(), Some("d"));
    assert_eq!(iter.nth_back(1).as_deref(), Some("b"));
    assert_eq!(iter.nth_back(100), None);
}

#[test]
fn extend() {
    let mut v = LightVec::<i32, 5>::new();
    v.extend([1, 2]);
    assert_eq!(v[..], [1, 2]);
    v.extend([3, 4]);
    assert_eq!(v[..], [1, 2, 3, 4]);
    v.extend([5, 6]);
    assert_eq!(v[..], [1, 2, 3, 4, 5, 6]);
}

#[test]
fn light_vec_macro() {
    let v: LightVec<i32, 5> = light_vec![];
    assert_eq!(v[..], []);
    let v: LightVec<i32, 5> = light_vec![1, 2, 3];
    assert_eq!(v[..], [1, 2, 3]);
    let v: LightVec<i32, 5> = light_vec![1, 2, 3, 4, 5, 6];
    assert_eq!(v[..], [1, 2, 3, 4, 5, 6]);
    let v: LightVec<i32, 0> = light_vec![];
    assert_eq!(v[..], []);
    let v: LightVec<i32, 0> = light_vec![1, 2, 3];
    assert_eq!(v[..], [1, 2, 3]);
    let v: LightVec<i32, 0> = light_vec![1, 2, 3, 4, 5, 6];
    assert_eq!(v[..], [1, 2, 3, 4, 5, 6]);
    let v: LightVec<String, 5> = light_vec!["A".into(), "a".into(), "B".into(), "b".into()];
    assert_eq!(v[..], ["A", "a", "B", "b"]);
}

#[test]
fn light_vec_macro_repeat() {
    let v: LightVec<i32, 5> = light_vec![123; 0];
    assert_eq!(v[..], []);
    let v: LightVec<String, 5> = light_vec!["a".into(); 3];
    assert_eq!(v[..], ["a", "a", "a"]);
    let v: LightVec<String, 5> = light_vec!["a".into(); 6];
    assert_eq!(v[..], ["a", "a", "a", "a", "a", "a"]);
}

#[test]
fn from_slice() {
    let v: LightVec<i32, 5> = [].as_slice().into();
    assert_eq!(v[..], []);
    let v: LightVec<i32, 5> = [1, 2, 3].as_slice().into();
    assert_eq!(v[..], [1, 2, 3]);
    let v: LightVec<i32, 5> = [1, 2, 3, 4, 5, 6].as_slice().into();
    assert_eq!(v[..], [1, 2, 3, 4, 5, 6]);
    let v: LightVec<String, 5> = [].as_slice().into();
    assert_eq!(v[..], *{
        const S: &[String] = &[];
        S
    });
    let v: LightVec<String, 5> = ["a".into(), "b".into(), "c".into()].as_slice().into();
    assert_eq!(v[..], ["a", "b", "c"]);
    let v: LightVec<String, 5> = [
        "a".into(),
        "b".into(),
        "c".into(),
        "d".into(),
        "e".into(),
        "f".into(),
    ]
    .as_slice()
    .into();
    assert_eq!(v[..], ["a", "b", "c", "d", "e", "f"]);
}

#[test]
fn retain() {
    let mut v: LightVec<i32, 5> = light_vec![];
    v.retain(|x| *x == 0);
    assert_eq!(v[..], []);
    let mut v: LightVec<String, 5> = light_vec!["A".into(), "a".into(), "B".into(), "b".into()];
    let mut u = v.clone();
    u.retain(|s| s.chars().next().unwrap().is_ascii_uppercase());
    assert_eq!(u[..], ["A", "B"]);
    v.retain(|s| s.chars().next().unwrap().is_ascii_lowercase());
    assert_eq!(v[..], ["a", "b"]);

    std::panic::catch_unwind(|| {
        let mut v: LightVec<String, 10> = "aaabbcc".chars().map(|c| c.to_string()).collect();
        v.retain(|x| {
            if *x == "c" {
                panic!();
            }
            true
        })
    })
    .unwrap_err();
}

#[test]
fn dedup() {
    let mut v: LightVec<String, 10> = LightVec::new();
    v.dedup();
    assert!(v.is_empty());
    let mut v: LightVec<String, 10> = "aaabbcc".chars().map(|c| c.to_string()).collect();
    v.dedup();
    assert_eq!(v[..], ["a", "b", "c"]);
    let mut v: LightVec<String, 10> = "abbbc".chars().map(|c| c.to_string()).collect();
    v.dedup();
    assert_eq!(v[..], ["a", "b", "c"]);
    let mut v: LightVec<String, 10> = "aaaaa".chars().map(|c| c.to_string()).collect();
    v.dedup();
    assert_eq!(v[..], ["a"]);

    let mut v: LightVec<i32, 10> = [1, 3, 5, 2, 4, 1].as_slice().into();
    v.dedup_by_key(|x| *x % 2);
    assert_eq!(v[..], [1, 2, 1]);

    std::panic::catch_unwind(|| {
        let mut v: LightVec<String, 10> = "aaabbcc".chars().map(|c| c.to_string()).collect();
        v.dedup_by_key(|x| {
            if *x == "c" {
                panic!();
            }
            x.chars().next()
        })
    })
    .unwrap_err();
}

#[test]
fn resize() {
    let mut v = LightVec::<String, 5>::new();
    v.resize(3, "a".to_string());
    assert_eq!(v[..], ["a", "a", "a"]);
    v.resize(2, "b".to_string());
    assert_eq!(v[..], ["a", "a"]);
    v.resize(6, "b".to_string());
    assert_eq!(v[..], ["a", "a", "b", "b", "b", "b"]);
    v.resize(4, "b".to_string());
    assert_eq!(v[..], ["a", "a", "b", "b"]);
}

#[test]
fn size() {
    use std::mem::size_of;
    assert_eq!(size_of::<LightVec<usize, 0>>(), 3 * size_of::<usize>());
    assert_eq!(size_of::<LightVec<usize, 2>>(), 3 * size_of::<usize>());
}
