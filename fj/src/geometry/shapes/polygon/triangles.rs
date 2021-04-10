use parry2d::shape::Triangle;

use super::Polygon;

pub struct Triangles<'r>(pub(super) &'r mut Polygon);

impl Triangles<'_> {
    pub fn remove(
        &mut self,
        triangle: Triangle,
    ) -> Result<(), TriangleNotPresent> {
        // TASK: Convert to update `self.edges` in addition to `self.chains`.
        //       Once this is done correctly, the other methods can be updated
        //       to use `self.edges`, and `self.chains` can be removed.

        // ---

        // Create a structure that gives us each point of the triangle together
        // with the two other points.
        let triangle = [
            (triangle.a, [triangle.b, triangle.c]),
            (triangle.b, [triangle.a, triangle.c]),
            (triangle.c, [triangle.a, triangle.b]),
        ];

        for chain in &mut self.0.chains {
            // Need to query a copy of the chain, else our removals will falsify
            // further queries.
            let chain_copy = chain.clone();

            for &(vertex, [a, b]) in &triangle {
                if let Some(neighbors) = chain_copy.neighbors_of(vertex) {
                    if neighbors.contains(a) && neighbors.contains(b) {
                        chain.remove(vertex);
                    }
                }
            }

            if chain.len() < chain_copy.len() {
                // We removed vertices from the chain.
                //
                // Due to the assumptions made by `Polygon` (no edges that
                // overlap, and no vertices shared between chains), we can
                // assume that we're done and will find nothing more.
                return Ok(());
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
    use parry2d::shape::Triangle;

    use crate::geometry::{
        segment::Seg2,
        shapes::{Polygon, VertexChain},
    };

    #[test]
    fn remove_should_remove_triangle() {
        let mut polygon = Polygon::new();

        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(1.0, 1.0);
        let d = Point2::new(0.0, 1.0);
        polygon.insert_chain(VertexChain::from(&[a, b, c, d][..]));

        polygon.triangles().remove(Triangle::new(b, c, d)).unwrap();
        assert_eq!(
            polygon.edges(),
            vec![Seg2::new(a, b), Seg2::new(b, d), Seg2::new(d, a)]
        );
    }

    #[test]
    fn remove_should_fail_if_not_all_vertices_are_in_polygon() {
        let mut polygon = Polygon::new();

        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(0.0, 1.0);
        polygon.insert_chain(VertexChain::from(&[a, b, c][..]));

        let triangle = Triangle::new(a, b, Point2::new(0.0, 2.0));
        assert!(polygon.triangles().remove(triangle).is_err());
    }

    #[test]
    fn remove_should_remove_all_vertices_if_necessary() {
        let mut polygon = Polygon::new();

        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(1.0, 1.0);
        polygon.insert_chain(VertexChain::from(&[a, b, c][..]));

        polygon.triangles().remove(Triangle::new(a, b, c)).unwrap();
        assert!(polygon.is_empty());
    }

    #[test]
    #[ignore]
    fn remove_should_remove_vertices_from_inner_and_outer_chain() {
        // TASK: Implement
        todo!()
    }
}
