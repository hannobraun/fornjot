use std::collections::HashSet;

use crate::geometry::shapes::{Seg2, Tri2};

use super::data::PolygonData;

pub struct Triangles<'r>(pub(super) &'r mut PolygonData);

impl Triangles<'_> {
    pub fn remove(
        &mut self,
        triangle: impl Into<Tri2>,
    ) -> Result<(), TriangleNotPresent> {
        let triangle = triangle.into();

        for vertex in &triangle.vertices() {
            if !self.0.contains_vertex(vertex) {
                return Err(TriangleNotPresent);
            }
        }

        // Convert triangle into a representation that is more useful for this
        // algorithm.
        let mut triangle_edges: HashSet<Seg2> = HashSet::new();
        for &edge in &triangle.edges() {
            triangle_edges.insert(edge);
        }

        // All edges that are fully contained in the triangle need to be
        // removed.
        self.0.retain_edges(|edge| {
            // TASK: Wether this works or not is dependent on the direction on
            //       the edge in the triangle. Make sure it works in any case.
            if triangle_edges.contains(edge) {
                // We need to remove this edge from the polygon. Also remove
                // it from `triangle_edges`, so it won't be processed in the
                // next step.
                triangle_edges.remove(edge);
                return false;
            }

            true
        });

        // All the triangle edges that haven't been removed, need to be added
        // to the polygon. Otherwise we're leaving a gap in the polygon edges.
        for edge in triangle_edges {
            // TASK: Make sure the edge has the correct direction. This one here
            //       just happens to work with the test we have.
            self.0.insert_edge(edge.reverse());
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct TriangleNotPresent;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::geometry::shapes::{Pnt2, Polygon, Seg2, Tri2};

    #[test]
    fn remove_should_remove_triangle() {
        let mut polygon = Polygon::new();

        let a = Pnt2::from_f32s(0.0, 0.0);
        let b = Pnt2::from_f32s(1.0, 0.0);
        let c = Pnt2::from_f32s(1.0, 1.0);
        let d = Pnt2::from_f32s(0.0, 1.0);
        polygon.insert_chain(&[a, b, c, d]);

        polygon.triangles().remove(Tri2::new(b, c, d)).unwrap();

        let mut expected = HashSet::new();
        expected.insert(Seg2::new(a, b));
        expected.insert(Seg2::new(b, d));
        expected.insert(Seg2::new(d, a));

        assert_eq!(polygon.edges(), &expected);
    }

    #[test]
    fn remove_should_fail_if_not_all_vertices_are_in_polygon() {
        let mut polygon = Polygon::new();

        let a = Pnt2::from_f32s(0.0, 0.0);
        let b = Pnt2::from_f32s(1.0, 0.0);
        let c = Pnt2::from_f32s(0.0, 1.0);
        polygon.insert_chain(&[a, b, c]);

        let triangle = Tri2::new(a, b, Pnt2::from_f32s(0.0, 2.0));
        assert!(polygon.triangles().remove(triangle).is_err());
    }

    #[test]
    fn remove_should_remove_all_vertices_if_necessary() {
        let mut polygon = Polygon::new();

        let a = Pnt2::from_f32s(0.0, 0.0);
        let b = Pnt2::from_f32s(1.0, 0.0);
        let c = Pnt2::from_f32s(1.0, 1.0);
        polygon.insert_chain(&[a, b, c]);

        polygon.triangles().remove(Tri2::new(a, b, c)).unwrap();
        assert!(polygon.is_empty());
    }

    #[test]
    #[ignore]
    fn remove_should_remove_vertices_from_inner_and_outer_chain() {
        // TASK: Implement
        todo!()
    }
}
