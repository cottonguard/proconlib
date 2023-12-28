// use crate::bounded::Bounded;
use std::ops::Deref;

#[derive(Clone)]
pub struct KdTree<T> {
    nodes: Vec<Node<T>>,
    min: (T, T),
    max: (T, T),
}

#[derive(Clone)]
pub struct Node<T> {
    pub pos: (T, T),
    pub left: usize,
    pub right: usize,
}

impl<T: Ord + Copy> KdTree<T> {
    pub fn new(mut points: Vec<(T, T)>) -> Self {
        let mut min = if let Some(p) = points.first() {
            *p
        } else {
            unimplemented!()
        };
        let mut max = min;
        for &(x, y) in &points[1..] {
            min.0 = min.0.min(x);
            min.1 = min.1.min(y);
            max.0 = max.0.max(x);
            max.1 = max.1.max(y);
        }
        let n = points.len();
        let mut kd_tree = Self {
            nodes: Vec::with_capacity(n),
            min,
            max,
        };
        kd_tree.new_rec(&mut points, 0);
        kd_tree.nodes.reverse();
        kd_tree
    }

    fn new_rec(&mut self, points: &mut [(T, T)], depth: u32) {
        points.sort_by(|(x1, y1), (x2, y2)| {
            if depth & 1 == 0 {
                x1.cmp(x2)
            } else {
                y1.cmp(y2)
            }
        });
        let med = points.len() / 2;
        let left = if med > 0 {
            self.new_rec(&mut points[..med], depth + 1);
            points.len() - self.nodes.len()
        } else {
            !0
        };
        let right = if med + 1 < points.len() {
            self.new_rec(&mut points[med + 1..], depth + 1);
            points.len() - self.nodes.len()
        } else {
            !0
        };
        self.nodes.push(Node {
            pos: points[med],
            left,
            right,
        });
    }

    pub fn cursor_root(&self) -> Cursor<T> {
        Cursor {
            tree: self,
            u: 0,
            depth: 0,
            min: self.min,
            max: self.max,
        }
    }
}

impl<T> Deref for KdTree<T> {
    type Target = [Node<T>];
    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

pub struct Cursor<'a, T> {
    tree: &'a KdTree<T>,
    u: usize,
    depth: u32,
    min: (T, T),
    max: (T, T),
}

impl<'a, T: Copy> Cursor<'a, T> {
    pub fn tree(&self) -> &KdTree<T> {
        self.tree
    }

    pub fn id(&self) -> usize {
        self.u
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }

    pub fn bound_rect(&self) -> ((T, T), (T, T)) {
        (self.min, self.max)
    }

    pub fn left(&self) -> Self {
        let max = if self.depth & 1 == 0 {
            (self.pos.0, self.max.1)
        } else {
            (self.max.0, self.pos.1)
        };
        Self {
            tree: self.tree,
            u: self.left,
            depth: self.depth + 1,
            min: self.min,
            max,
        }
    }

    pub fn right(&self) -> Self {
        let min = if self.depth & 1 == 0 {
            (self.pos.0, self.min.1)
        } else {
            (self.min.0, self.pos.1)
        };
        Self {
            tree: self.tree,
            u: self.right,
            depth: self.depth + 1,
            min,
            max: self.max,
        }
    }
}

impl<'a, T> Deref for Cursor<'a, T> {
    type Target = Node<T>;
    fn deref(&self) -> &Node<T> {
        &self.tree[self.u]
    }
}
