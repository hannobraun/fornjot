pub mod data;
pub mod triangles;
pub mod vertices;

use std::collections::HashSet;

use crate::geometry::segment::Seg2;

use self::{data::PolygonData, triangles::Triangles, vertices::Vertices};

use super::VertexChain;

/// A polygon
///
/// A polygon expects, but does not enforce, that none of its edges overlap, and
/// that none of its vertex chains share vertices.
#[derive(Clone, Debug)]
pub struct Polygon {
    // TASK: This representation is not flexible enough. It can't handle vertex
    //       chains sharing vertices, but this is a valid case that can occur
    //       when removing triangles from the polygon.
    //
    //       Add alternative fields that store the edges instead of vertex
    //       chains, then remove this one.
    pub chains: Vec<VertexChain>,

    pub data: PolygonData,
}

impl Polygon {
    pub fn new() -> Self {
        Self {
            chains: Vec::new(),
            data: PolygonData {
                edges: HashSet::new(),
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data.edges.is_empty()
    }

    pub fn insert_chain(&mut self, chain: VertexChain) {
        for segment in chain.segments() {
            self.data.edges.insert(segment.into());
        }
        self.chains.push(chain);
    }

    pub fn edges(&self) -> HashSet<Seg2> {
        self.data.edges.clone()
    }

    pub fn vertices(&mut self) -> Vertices {
        Vertices(self)
    }

    pub fn triangles(&mut self) -> Triangles {
        Triangles(self)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use nalgebra::Point2;

    use crate::geometry::{segment::Seg2, shapes::VertexChain};

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
        polygon.insert_chain(VertexChain::from(&[a, b, c][..]));
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

        let chain_a = VertexChain::from(&[a, b, c][..]);
        let chain_b = VertexChain::from(&[p, q, r][..]);

        polygon.insert_chain(chain_a);
        polygon.insert_chain(chain_b);

        let mut expected = HashSet::new();
        expected.insert(Seg2::new(a, b));
        expected.insert(Seg2::new(b, c));
        expected.insert(Seg2::new(c, a));
        expected.insert(Seg2::new(p, q));
        expected.insert(Seg2::new(q, r));
        expected.insert(Seg2::new(r, p));

        assert_eq!(polygon.edges(), expected);
    }
}
