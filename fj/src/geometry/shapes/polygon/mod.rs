pub mod data;
pub mod triangles;
pub mod vertices;

use core::fmt;
use std::collections::BTreeSet;

use crate::geometry::shapes::{Pnt, Seg2};

use self::{data::PolygonData, triangles::Triangles, vertices::Vertices};

/// A polygon
///
/// A polygon expects, but does not enforce, that none of its edges overlap, and
/// that none of its vertex chains share vertices.
#[derive(Clone, Debug)]
pub struct Polygon(PolygonData);

impl Polygon {
    pub fn new() -> Self {
        Self(PolygonData::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert_chain(
        &mut self,
        chain: impl IntoIterator<Item = impl Into<Pnt<2>>>,
    ) {
        // This gets us access to the `windows` method.
        let chain: Vec<_> =
            chain.into_iter().map(|vertex| vertex.into()).collect();

        for window in chain.windows(2) {
            let a = window[0];
            let b = window[1];

            self.0.insert_edge(Seg2::new(a, b));
        }

        // TASK: Handle the case that `first` and `last` are equal (i.e. there
        //       is only one vertex).
        if let (Some(first), Some(last)) = (chain.first(), chain.last()) {
            self.0.insert_edge(Seg2::new(last, first));
        }
    }

    pub fn edges(&self) -> &BTreeSet<Seg2> {
        self.0.edges()
    }

    pub fn reverse(&mut self) {
        self.0.reverse()
    }

    pub fn merge(&mut self, other: Self) {
        self.0.merge(other.0)
    }

    pub fn vertices(&mut self) -> Vertices {
        Vertices(&mut self.0)
    }

    pub fn triangles(&mut self) -> Triangles {
        Triangles(&mut self.0)
    }
}

impl fmt::Display for Polygon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Polygon: ")?;

        for edge in self.edges() {
            writeln!(f, "\t{}", edge)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use crate::geometry::shapes::{Pnt, Seg2};

    use super::Polygon;

    #[test]
    fn polygon_should_tell_whether_it_is_empty() {
        let mut polygon = Polygon::new();
        assert!(polygon.is_empty());

        let empty_chain: &[Pnt<2>] = &[];
        polygon.insert_chain(empty_chain);

        // Empty chain added, polygon still empty.
        assert!(polygon.is_empty());

        let a = Pnt::new(0.0, 0.0);
        let b = Pnt::new(1.0, 0.0);
        let c = Pnt::new(1.0, 1.0);

        // Non-empty chain, ergo polygon no longer empty.
        polygon.insert_chain(&[a, b, c]);
        assert!(!polygon.is_empty());
    }

    #[test]
    fn polygon_should_return_its_edges() {
        let mut polygon = Polygon::new();

        let a = Pnt::new(0.0, 0.0);
        let b = Pnt::new(2.0, 0.0);
        let c = Pnt::new(0.0, 2.0);

        let p = Pnt::new(1.0, 1.0);
        let q = Pnt::new(1.0, 1.5);
        let r = Pnt::new(1.5, 1.0);

        polygon.insert_chain(&[a, b, c]);
        polygon.insert_chain(&[p, q, r]);

        let mut expected = BTreeSet::new();
        expected.insert(Seg2::new(a, b));
        expected.insert(Seg2::new(b, c));
        expected.insert(Seg2::new(c, a));
        expected.insert(Seg2::new(p, q));
        expected.insert(Seg2::new(q, r));
        expected.insert(Seg2::new(r, p));

        assert_eq!(polygon.edges(), &expected);
    }

    #[test]
    fn reverse_should_reverse_all_edges() {
        let mut polygon = Polygon::new();

        let a = Pnt::new(0.0, 0.0);
        let b = Pnt::new(1.0, 0.0);
        let c = Pnt::new(1.0, 1.0);
        polygon.insert_chain(&[a, b, c]);

        polygon.reverse();

        let edges = polygon.edges();
        assert!(edges.contains(&Seg2::new(a, c)));
        assert!(edges.contains(&Seg2::new(c, b)));
        assert!(edges.contains(&Seg2::new(b, a)));
    }
}
