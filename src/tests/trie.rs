mod aho_corasick;

use crate::trie::*;

#[test]
fn dists() {
    let mut trie = Trie::new();
    trie.add(b"a!", 0);
    trie.add(b"aa", 1);
    trie.add(b"ab", 2);
    trie.add(b"ac", 3);
    trie.add(&[b'a', 255], 4);
    assert_eq!(
        trie.dests(1).collect::<Vec<_>>(),
        [(b'!', 2), (b'a', 3), (b'b', 4), (b'c', 5), (255, 6)]
    );
}
