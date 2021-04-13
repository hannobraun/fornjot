pub mod data;
pub mod triangles;
pub mod vertices;

use std::collections::HashSet;

use crate::geometry::{point::Pnt2, segment::Seg2};

use self::{data::PolygonData, triangles::Triangles, vertices::Vertices};

/// A polygon
///
/// A polygon expects, but does not enforce, that none of its edges overlap, and
/// that none of its vertex chains share vertices.
// TASK: Convert into tuple struct.
#[derive(Clone, Debug)]
pub struct Polygon {
    data: PolygonData,
}

impl Polygon {
    pub fn new() -> Self {
        Self {
            data: PolygonData::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn insert_chain(
        &mut self,
        chain: impl IntoIterator<Item = impl Into<Pnt2>>,
    ) {
        // This gets us access to the `windows` method.
        let chain: Vec<_> =
            chain.into_iter().map(|vertex| vertex.into()).collect();

        for window in chain.windows(2) {
            let a = window[0];
            let b = window[1];

            self.data.insert_edge(Seg2::new(a, b));
        }

        // TASK: Handle the case that `first` and `last` are equal (i.e. there
        //       is only one vertex).
        if let (Some(first), Some(last)) = (chain.first(), chain.last()) {
            self.data.insert_edge(Seg2::new(last, first));
        }
    }

    pub fn edges(&self) -> &HashSet<Seg2> {
        self.data.edges()
    }

    pub fn vertices(&mut self) -> Vertices {
        Vertices(&mut self.data)
    }

    pub fn triangles(&mut self) -> Triangles {
        Triangles(&mut self.data)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use nalgebra::Point2;

    use crate::geometry::segment::Seg2;

    use super::Polygon;

    #[test]
    fn polygon_should_tell_whether_it_is_empty() {
        let mut polygon = Polygon::new();
        assert!(polygon.is_empty());

        // Empty chain, polygon still empty.
        assert!(polygon.is_empty());

        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(1.0, 1.0);

        // Non-empty chain, ergo polygon no longer empty.
        polygon.insert_chain(&[a, b, c]);
        assert!(!polygon.is_empty());
    }

    #[test]
    fn polygon_chain_return_its_edges() {
        let mut polygon = Polygon::new();

        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(2.0, 0.0);
        let c = Point2::new(0.0, 2.0);

        let p = Point2::new(1.0, 1.0);
        let q = Point2::new(1.0, 1.5);
        let r = Point2::new(1.5, 1.0);

        polygon.insert_chain(&[a, b, c]);
        polygon.insert_chain(&[p, q, r]);

        let mut expected = HashSet::new();
        expected.insert(Seg2::new(a, b));
        expected.insert(Seg2::new(b, c));
        expected.insert(Seg2::new(c, a));
        expected.insert(Seg2::new(p, q));
        expected.insert(Seg2::new(q, r));
        expected.insert(Seg2::new(r, p));

        assert_eq!(polygon.edges(), &expected);
    }
}
