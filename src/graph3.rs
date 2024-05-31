type Idx = u32;

pub struct Graph {
    heads: Vec<Idx>,
    tails: Vec<Idx>,
    edges: Vec<EdgeData>,
}

struct EdgeData {
    to: Idx,
    next: Idx,
}

impl Graph {
    
}