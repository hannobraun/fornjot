use crate::geometry::shapes::{Pnt2, Seg2};

/// The edge of a polygon
///
/// In contrast to line segment, polygon edges are always undirected, meaning
/// the same two vertices will always create the same polygon edge.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Edge {
    /// The "lesser" vertex
    a: Pnt2,

    /// The "greater" vertex
    b: Pnt2,
}

impl Edge {
    pub fn new(a: Pnt2, b: Pnt2) -> Self {
        if a < b {
            Self { a: a, b: b }
        } else {
            Self { a: b, b: a }
        }
    }

    pub fn a(&self) -> Pnt2 {
        self.a
    }

    pub fn b(&self) -> Pnt2 {
        self.b
    }
}

impl From<Seg2> for Edge {
    fn from(segment: Seg2) -> Self {
        Self::new(segment.a, segment.b)
    }
}
