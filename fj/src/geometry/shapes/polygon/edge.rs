use crate::geometry::shapes::{Pnt2, Seg2};

/// The edge of a polygon
///
/// In contrast to line segment, polygon edges are always undirected, meaning
/// the same two vertices will always create the same polygon edge.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Edge {
    /// The "lesser" vertex
    a: Pnt2,

    /// The "greater" vertex
    b: Pnt2,
}

impl Edge {
    pub fn a(&self) -> Pnt2 {
        self.a
    }

    pub fn b(&self) -> Pnt2 {
        self.b
    }
}

impl From<Seg2> for Edge {
    fn from(segment: Seg2) -> Self {
        if segment.a < segment.b {
            Self {
                a: segment.a,
                b: segment.b,
            }
        } else {
            Self {
                a: segment.b,
                b: segment.a,
            }
        }
    }
}
