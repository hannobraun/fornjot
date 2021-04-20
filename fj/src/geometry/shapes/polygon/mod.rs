pub mod data;
pub mod triangles;
pub mod vertices;

use std::collections::HashSet;

use crate::geometry::shapes::{Pnt2, Seg2};

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

    /// Returns the first vertex, as defined by `Ord`/`PartialOrd`
    pub fn first_vertex(&self) -> Option<Pnt2> {
        self.0.first_vertex()
    }

    pub fn neighbors_of(&self, vertex: &Pnt2) -> Option<HashSet<Pnt2>> {
        let mut neighbors = HashSet::new();

        for outgoing in self.0.outgoing_edges(vertex)? {
            neighbors.insert(outgoing.b);
        }
        for incoming in self.0.incoming_edges(vertex)? {
            neighbors.insert(incoming.a);
        }

        Some(neighbors)
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

            self.0.insert_edge(Seg2::new(a, b));
        }

        // TASK: Handle the case that `first` and `last` are equal (i.e. there
        //       is only one vertex).
        if let (Some(first), Some(last)) = (chain.first(), chain.last()) {
            self.0.insert_edge(Seg2::new(last, first));
        }
    }

    pub fn edges(&self) -> &HashSet<Seg2> {
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::geometry::shapes::{Pnt2, Seg2};

    use super::Polygon;

    #[test]
    fn polygon_should_tell_whether_it_is_empty() {
        let mut polygon = Polygon::new();
        assert!(polygon.is_empty());

        let empty_chain: &[Pnt2] = &[];
        polygon.insert_chain(empty_chain);

        // Empty chain added, polygon still empty.
        assert!(polygon.is_empty());

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);
        let c = Pnt2::new(1.0, 1.0);

        // Non-empty chain, ergo polygon no longer empty.
        polygon.insert_chain(&[a, b, c]);
        assert!(!polygon.is_empty());
    }

    #[test]
    fn polygon_should_return_its_edges() {
        let mut polygon = Polygon::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(2.0, 0.0);
        let c = Pnt2::new(0.0, 2.0);

        let p = Pnt2::new(1.0, 1.0);
        let q = Pnt2::new(1.0, 1.5);
        let r = Pnt2::new(1.5, 1.0);

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

    #[test]
    fn neighbors_of_should_return_neighbors_of_vertex() {
        let mut polygon = Polygon::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(2.0, 0.0);
        let c = Pnt2::new(2.0, 2.0);
        let d = Pnt2::new(0.0, 2.0);

        let e = Pnt2::new(1.0, 0.5);
        let f = Pnt2::new(1.0, 1.0);
        let g = Pnt2::new(0.5, 1.0);

        polygon.insert_chain(&[a, b, c, d]);
        polygon.insert_chain(&[a, e, f, g]);

        let neighbors_of_a = polygon.neighbors_of(&a).unwrap();

        assert_eq!(neighbors_of_a.contains(&a), false);
        assert_eq!(neighbors_of_a.contains(&b), true);
        assert_eq!(neighbors_of_a.contains(&c), false);
        assert_eq!(neighbors_of_a.contains(&d), true);
        assert_eq!(neighbors_of_a.contains(&e), true);
        assert_eq!(neighbors_of_a.contains(&f), false);
        assert_eq!(neighbors_of_a.contains(&g), true);
    }

    #[test]
    fn reverse_should_reverse_all_edges() {
        let mut polygon = Polygon::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);
        let c = Pnt2::new(1.0, 1.0);
        polygon.insert_chain(&[a, b, c]);

        polygon.reverse();

        let edges = polygon.edges();
        assert!(edges.contains(&Seg2::new(a, c)));
        assert!(edges.contains(&Seg2::new(c, b)));
        assert!(edges.contains(&Seg2::new(b, a)));
    }
}
