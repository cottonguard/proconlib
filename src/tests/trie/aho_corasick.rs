use crate::trie::{aho_corasick::*, Trie};

#[test]
fn find_longest() {
    {
        let mut trie = Trie::new();
        trie.add(b"a", 1);
        trie.add(b"ab", 2);
        trie.add(b"bc", 3);
        trie.add(b"c", 4);
        let ah = AhoCorasick::new(trie);
        assert_eq!(
            ah.find_longest_suffix(b"abc").collect::<Vec<_>>(),
            [(0, 1, &1), (0, 2, &2), (1, 3, &3)]
        );
    }
    {
        let mut trie = Trie::new();
        trie.add(b"", 1);
        let ah = AhoCorasick::new(trie);
        assert_eq!(
            ah.find_longest_suffix(b"abc").collect::<Vec<_>>(),
            [(0, 0, &1), (1, 1, &1), (2, 2, &1), (3, 3, &1)]
        );
    }
    {
        let trie = Trie::<i32>::new();
        let ah = AhoCorasick::new(trie);
        assert_eq!(ah.find_longest_suffix(b"abc").collect::<Vec<_>>(), []);
    }
    {
        let mut trie = Trie::new();
        trie.add(b"bc", 1);
        trie.add(b"abcd", 2);
        let ah = AhoCorasick::new(trie);
        assert_eq!(
            ah.find_longest_suffix(b"abcd").collect::<Vec<_>>(),
            [(1, 3, &1), (0, 4, &2)]
        );
    }
    {
        let s = b"3141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067982148086513282306647093844609550582231725359408128481117450284102701938521105559";
        let mut trie = Trie::new();
        trie.add(b"314159", 1);
        trie.add(b"1592653", 2);
        trie.add(b"415926535897", 3);
        trie.add(b"897932", 4);
        trie.add(b"643383", 5);
        let ah = AhoCorasick::new(trie);
        assert_eq!(
            ah.find_longest_suffix(s).collect::<Vec<_>>(),
            [
                (0, 6, &1),
                (3, 10, &2),
                (2, 14, &3),
                (11, 17, &4),
                (22, 28, &5)
            ]
        );
    }
}

#[test]
fn suffixes() {
    let mut trie = Trie::new();
    let a = trie.add(b"a", 1).0;
    let ba = trie.add(b"ba", 2).0;
    let cba = trie.add(b"ccccccccba", 3).0;
    trie.add(b"c", 4);
    let ah = AhoCorasick::new(trie);
    assert_eq!(ah.suffixes_by_node_id(a).collect::<Vec<_>>(), [(a, &1)]);
    assert_eq!(
        ah.suffixes_by_node_id(ba).collect::<Vec<_>>(),
        [(ba, &2), (a, &1)]
    );
    assert_eq!(
        ah.suffixes_by_node_id(cba).collect::<Vec<_>>(),
        [(cba, &3), (ba, &2), (a, &1)]
    );
}
