use std::{cmp::Ordering, mem, ops};

pub struct LiChaoTree<T> {
    root: Node<T>,
    min_x: T,
    max_x: T,
}

struct Node<T>(Option<Box<NodeInner<T>>>);

struct NodeInner<T> {
    x: T,
    line: (T, T),
    left: Node<T>,
    right: Node<T>,
}

impl<T: Num> LiChaoTree<T> {
    pub fn new() -> Self {
        let (min_x, max_x) = T::default_range();
        Self {
            root: Node::nil(),
            max_x,
            min_x,
        }
    }

    pub fn y(&self, x: T) -> T {
        self.try_y(x).expect("no line")
    }

    pub fn try_y(&self, x: T) -> Option<T> {
        self.root.y(x, |x| x)
    }

    pub fn y_wide(&self, x: T) -> T::Wide {
        self.try_y_wide(x).expect("no line")
    }

    pub fn try_y_wide(&self, x: T) -> Option<T::Wide> {
        self.root.y(x, |x| x.to_wide())
    }

    pub fn add_line(&mut self, line: (T, T)) -> &mut Self {
        self.root.add_line(self.min_x, self.max_x, line);
        self
    }
}

impl<T: Num> Default for LiChaoTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Num> Node<T> {
    fn nil() -> Self {
        Self(None)
    }

    fn y<U: NumBase>(&self, x: T, f: impl Fn(T) -> U) -> Option<U> {
        if let Some(node) = &self.0 {
            let y = f(node.line.0) * f(x) + f(node.line.1);

            let y_child = match x.cmp(&node.x) {
                Ordering::Less => node.left.y(x, f),
                Ordering::Greater => node.right.y(x, f),
                Ordering::Equal => None,
            };

            Some(if let Some(y_child) = y_child {
                y.min(y_child)
            } else {
                y
            })
        } else {
            None
        }
    }

    fn add_line(&mut self, x_left: T, x_right: T, mut line: (T, T)) {
        if let Some(node) = &mut self.0 {
            if line.0 == node.line.0 {
                if line.1 < node.line.1 {
                    node.line = line;
                }
                return;
            }

            if (line.0 - node.line.0).to_wide() * node.x.to_wide()
                < (node.line.1 - line.1).to_wide()
            {
                mem::swap(&mut line, &mut node.line);
            }

            // a1x + b1 = a2x + b2
            // x = (b2 - b1) / (a1 - a2)
            // x < m
            // (b2 - b1) / (a1 - a2) < m
            // b2 - b1 < (a1 - a2)m
            let ((a1, b1), (a2, b2)) = if line.0 > node.line.0 {
                (line, node.line)
            } else {
                (node.line, line)
            };
            let c = (b2 - b1)
                .to_wide()
                .cmp(&((a1 - a2).to_wide() * node.x.to_wide()));

            match c {
                Ordering::Less => {
                    if (b2 - b1).to_wide() > (a1 - a2).to_wide() * x_left.to_wide() {
                        node.left.add_line(x_left, node.x, line)
                    }
                }
                Ordering::Greater => {
                    if (b2 - b1).to_wide() < (a1 - a2).to_wide() * x_right.to_wide() {
                        node.right.add_line(node.x, x_right, line)
                    }
                }
                Ordering::Equal => {}
            }
        } else {
            if x_right - x_left <= T::eps() {
                return;
            }
            self.0 = Some(Box::new(NodeInner {
                x: T::midpoint(x_left, x_right),
                line,
                left: Self::nil(),
                right: Self::nil(),
            }));
        }
    }
}

pub trait Num: NumBase {
    type Wide: From<Self> + NumBase;

    fn midpoint(self, other: Self) -> Self;
    fn eps() -> Self;
    fn default_range() -> (Self, Self);
    fn to_wide(self) -> Self::Wide {
        Self::Wide::from(self)
    }
}

pub trait NumBase:
    Copy
    + Ord
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + std::fmt::Debug
{
}

impl Num for i32 {
    type Wide = i64;

    fn midpoint(self, other: Self) -> Self {
        self + (other - self) / 2
    }

    fn eps() -> Self {
        1
    }

    fn default_range() -> (Self, Self) {
        (-(1 << 29), 1 << 29)
    }
}

impl Num for i64 {
    type Wide = i128;

    fn midpoint(self, other: Self) -> Self {
        self + (other - self) / 2
    }

    fn eps() -> Self {
        1
    }

    fn default_range() -> (Self, Self) {
        (-(1 << 61), 1 << 61)
    }
}

impl NumBase for i32 {}
impl NumBase for i64 {}
impl NumBase for i128 {}
