pub mod triangles;

use parry2d::shape::Segment;

use crate::geometry::segment::Seg2;

use self::triangles::Triangles;

use super::VertexChain;

// TASK: I'd like to add all kinds of functionality to `Polygon`, like querying
//       the neighbors of vertices, or removing triangles. This will overload
//       this module, and adding `impl`s over multiple modules doesn't seem like
//       a good idea.
//
//       How about this instead: `Polygon` has methods that return new APIs,
//       which borrow its innards. E.g. `polygon.triangles().remove(triangle)`.

/// A polygon
///
/// A polygon expects, but does not enforce, that none of its edges overlap, and
/// that none of its vertex chains share vertices.
#[derive(Clone, Debug)]
pub struct Polygon(PolygonInner);

impl Polygon {
    pub fn new() -> Self {
        Self(PolygonInner {
            chains: Vec::new(),
            edges: Vec::new(),
        })
    }

    pub fn is_empty(&self) -> bool {
        // TASK: Convert to use `self.edges`.

        for chain in &self.0.chains {
            if !chain.is_empty() {
                return false;
            }
        }

        true
    }

    pub fn insert_chain(&mut self, chain: VertexChain) {
        for segment in chain.segments() {
            self.0.edges.push(segment.into());
        }
        self.0.chains.push(chain);
    }

    pub fn edges(&self) -> Vec<Segment> {
        // TASK: Convert to use `self.edges`.

        let mut edges = Vec::new();

        for chain in &self.0.chains {
            edges.extend_from_slice(&chain.segments());
        }

        edges
    }

    pub fn triangles(&mut self) -> Triangles {
        Triangles(&mut self.0)
    }
}

#[derive(Clone, Debug)]
pub struct PolygonInner {
    // TASK: This representation is not flexible enough. It can't handle vertex
    //       chains sharing vertices, but this is a valid case that can occur
    //       when removing triangles from the polygon.
    //
    //       Add alternative fields that store the edges instead of vertex
    //       chains, then remove this one.
    pub chains: Vec<VertexChain>,
    pub edges: Vec<Seg2>,
}

#[cfg(test)]
mod tests {
    use nalgebra::Point2;
    use parry2d::shape::Segment;

    use crate::geometry::shapes::VertexChain;

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

        assert_eq!(
            polygon.edges(),
            vec![
                Segment::new(a, b),
                Segment::new(b, c),
                Segment::new(c, a),
                Segment::new(p, q),
                Segment::new(q, r),
                Segment::new(r, p)
            ]
        );
    }
}
