pub struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::new()],
        }
    }

    #[inline]
    fn find_node(&self, s: &[u8]) -> (usize, usize) {
        let mut node = 0;
        let mut len = 0;
        for &b in s {
            if let Some(dest) = self.nodes[node].dest(b) {
                node = dest;
            } else {
                break;
            }
            len += 1;
        }
        (node, len)
    }

    pub fn add(&mut self, pat: &[u8]) {
        let (mut node, len) = self.find_node(pat);
        for &b in &pat[len..] {
            let next = self.nodes.len();
            self.nodes[node].push(b, next);
            self.nodes.push(Node::new());
            node = next;
        }
    }
}

struct Node {
    bytes: Vec<u8>,
    dests: Vec<usize>,
}

impl Node {
    fn new() -> Self {
        Self {
            bytes: vec![],
            dests: vec![],
        }
    }

    fn push(&mut self, b: u8, dest: usize) {
        self.bytes.push(b);
        self.dests.push(dest);
    }

    fn dest(&self, b: u8) -> Option<usize> {
        self.bytes
            .iter()
            .position(|c| b == *c)
            .map(|i| self.dests[i])
    }
}

#[derive(Clone, Copy)]
pub struct State<'a> {
    trie: &'a Trie,
    node: usize,
}

impl<'a> State<'a> {
    pub fn transition(&mut self, b: u8) -> Option<usize> {
        self.node = self
            .trie
            .nodes
            .get(self.node)
            .and_then(|node| node.dest(b))
            .unwrap_or(!0);
        self.node()
    }

    pub fn node(&self) -> Option<usize> {
        if self.node < self.trie.nodes.len() {
            Some(self.node)
        } else {
            None
        }
    }
}
