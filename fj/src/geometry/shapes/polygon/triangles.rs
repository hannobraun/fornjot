use std::collections::HashSet;

use crate::geometry::shapes::Tri2;

use super::{data::PolygonData, edge::Edge};

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
        let mut triangle_edges: HashSet<Edge> = HashSet::new();
        for &edge in &triangle.edges() {
            triangle_edges.insert(edge.into());
        }

        // All edges that are fully contained in the triangle need to be
        // removed.
        self.0.retain_edges(|edge| {
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
            self.0.insert_edge(edge);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct TriangleNotPresent;

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::geometry::shapes::{polygon::Edge, Pnt2, Polygon, Tri2};

    #[test]
    fn remove_should_remove_triangle() {
        let mut polygon = Polygon::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);
        let c = Pnt2::new(1.0, 1.0);
        let d = Pnt2::new(0.0, 1.0);
        polygon.insert_chain(&[a, b, c, d]);

        polygon.triangles().remove(Tri2::new(b, c, d)).unwrap();

        let mut expected = HashSet::new();
        expected.insert(Edge::new(a, b));
        expected.insert(Edge::new(b, d));
        expected.insert(Edge::new(d, a));

        assert_eq!(polygon.edges(), &expected);
    }

    // TASK: Un-ignore.
    #[test]
    #[ignore]
    fn remove_should_remove_triangle_from_hole_polygon() {
        let mut polygon = Polygon::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);
        let c = Pnt2::new(1.0, 1.0);
        let d = Pnt2::new(0.0, 1.0);

        // Polygon is defined clock-wise, which indicates a hole, not an actual
        // positive polygon.
        polygon.insert_chain(&[a, d, c, b]);

        polygon.triangles().remove(Tri2::new(b, c, d)).unwrap();

        let mut expected = HashSet::new();
        expected.insert(Edge::new(a, d));
        expected.insert(Edge::new(d, b));
        expected.insert(Edge::new(b, a));

        assert_eq!(polygon.edges(), &expected);
    }

    #[test]
    fn remove_should_fail_if_not_all_vertices_are_in_polygon() {
        let mut polygon = Polygon::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);
        let c = Pnt2::new(0.0, 1.0);
        polygon.insert_chain(&[a, b, c]);

        let triangle = Tri2::new(a, b, Pnt2::new(0.0, 2.0));
        assert!(polygon.triangles().remove(triangle).is_err());
    }

    #[test]
    fn remove_should_remove_all_vertices_if_necessary() {
        let mut polygon = Polygon::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);
        let c = Pnt2::new(1.0, 1.0);
        polygon.insert_chain(&[a, b, c]);

        polygon.triangles().remove(Tri2::new(a, b, c)).unwrap();
        assert!(polygon.is_empty());
    }

    #[test]
    fn remove_should_remove_vertices_from_inner_and_outer_chain() {
        let mut polygon = Polygon::new();

        // Outer perimeter
        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(2.0, 0.0);
        let c = Pnt2::new(2.0, 2.0);
        let d = Pnt2::new(0.0, 2.0);
        polygon.insert_chain(&[a, b, c, d]);

        // Inner perimeter (hole)
        let x = Pnt2::new(0.5, 0.5);
        let y = Pnt2::new(0.5, 1.5);
        let z = Pnt2::new(1.5, 1.5);
        let w = Pnt2::new(1.5, 0.5);
        polygon.insert_chain(&[x, y, z, w]);

        polygon.triangles().remove(Tri2::new(a, x, d)).unwrap();

        let mut expected = HashSet::new();
        expected.insert(Edge::new(a, b));
        expected.insert(Edge::new(b, c));
        expected.insert(Edge::new(c, d));
        expected.insert(Edge::new(d, x));
        expected.insert(Edge::new(x, y));
        expected.insert(Edge::new(y, z));
        expected.insert(Edge::new(z, w));
        expected.insert(Edge::new(w, x));
        expected.insert(Edge::new(x, a));

        assert_eq!(polygon.edges(), &expected);
    }
}
