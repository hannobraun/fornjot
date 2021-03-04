use parry2d::shape::{Segment, Triangle};

use super::VertexChain;

/// A polygon
///
/// A polygon expects, but does not enforce, that none of its edges overlap, and
/// that none of its vertex chains share vertices.
pub struct Polygon(Vec<VertexChain>);

impl Polygon {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn insert_chain(&mut self, chain: VertexChain) {
        self.0.push(chain)
    }

    pub fn edges(&self) -> Vec<Segment> {
        let mut edges = Vec::new();

        for chain in &self.0 {
            edges.extend_from_slice(&chain.segments());
        }

        edges
    }

    pub fn remove_triangle(
        &mut self,
        triangle: Triangle,
    ) -> Result<(), TriangleNotPresent> {
        let triangle = [
            (triangle.a, [triangle.b, triangle.c]),
            (triangle.b, [triangle.a, triangle.c]),
            (triangle.c, [triangle.a, triangle.b]),
        ];
        for &(vertex, [a, b]) in &triangle {
            for chain in &mut self.0 {
                if let Some(neighbors) = chain.neighbors_of(vertex) {
                    if neighbors.contains(a) && neighbors.contains(b) {
                        chain.remove(vertex);

                        // Due to the assumptions made by `Polygon` (no edges
                        // that overlap, and no vertices shared between chains),
                        // we can assume that we're done and will find nothing
                        // more.
                        return Ok(());
                    }
                }
            }
        }

        Err(TriangleNotPresent)
    }
}

#[derive(Debug)]
pub struct TriangleNotPresent;

#[cfg(test)]
mod tests {
    use nalgebra::Point2;
    use parry2d::shape::{Segment, Triangle};

    use crate::geometry::shapes::VertexChain;

    use super::Polygon;

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

    #[test]
    fn polygon_should_remove_triangle() {
        let mut polygon = Polygon::new();

        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(1.0, 1.0);
        let d = Point2::new(0.0, 1.0);
        polygon.insert_chain(VertexChain::from(&[a, b, c, d][..]));

        polygon.remove_triangle(Triangle::new(b, c, d)).unwrap();
        assert_eq!(
            polygon.edges(),
            vec![Segment::new(a, b), Segment::new(b, d), Segment::new(d, a)]
        );
    }

    #[test]
    fn polygon_should_fail_if_triangle_points_are_not_fully_contained() {
        let mut polygon = Polygon::new();

        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(0.0, 1.0);
        polygon.insert_chain(VertexChain::from(&[a, b, c][..]));

        let triangle = Triangle::new(a, b, Point2::new(0.0, 2.0));
        assert!(polygon.remove_triangle(triangle).is_err());
    }
}
