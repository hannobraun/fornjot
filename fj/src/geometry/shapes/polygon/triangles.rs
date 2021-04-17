use std::collections::HashSet;

use crate::geometry::shapes::{Pnt2, Seg2, Tri2};

use super::data::PolygonData;

pub struct Triangles<'r>(pub(super) &'r mut PolygonData);

impl Triangles<'_> {
    pub fn remove(&mut self, triangle: impl Into<Tri2>) -> Result<(), Error> {
        let triangle = triangle.into();

        for &vertex in &triangle.vertices() {
            if !self.0.contains_vertex(&vertex) {
                return Err(Error::UnknownVertex(vertex));
            }
        }

        // TASK: Enable this test.
        // for &edge in &triangle.edges() {
        //     if !self.0.is_inside(&edge).unwrap() {
        //         return Err(Error::OutsideOfPolygon(edge));
        //     }
        // }

        // Convert triangle into a representation that is more useful for this
        // algorithm.
        let mut triangle_edges: HashSet<Seg2> = HashSet::new();
        for &edge in &triangle.edges() {
            triangle_edges.insert(edge);
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
            if triangle_edges.contains(&edge.reverse()) {
                // We need to remove this edge from the polygon. Also remove
                // it from `triangle_edges`, so it won't be processed in the
                // next step.
                triangle_edges.remove(&edge.reverse());
                return false;
            }

            true
        });

        // All the triangle edges that haven't been removed, need to be added
        // to the polygon. Otherwise we're leaving a gap in the polygon edges.
        for edge in triangle_edges {
            let a_incoming = self.0.incoming_edges(&edge.a).unwrap().len();
            let a_outgoing = self.0.outgoing_edges(&edge.a).unwrap().len();
            let b_incoming = self.0.incoming_edges(&edge.b).unwrap().len();
            let b_outgoing = self.0.outgoing_edges(&edge.b).unwrap().len();

            if a_outgoing < a_incoming || b_incoming < b_outgoing {
                self.0.insert_edge(edge);
                continue;
            }
            if a_outgoing > a_incoming || b_incoming > b_outgoing {
                self.0.insert_edge(edge.reverse());
                continue;
            }

            unreachable!("All vertices are balanced.");
        }

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    OutsideOfPolygon(Seg2),
    UnknownVertex(Pnt2),
}

impl Error {
    pub fn is_outside_of_polygon(&self) -> bool {
        match self {
            Self::OutsideOfPolygon(_) => true,
            _ => false,
        }
    }

    pub fn is_unknown_vertex(&self) -> bool {
        match self {
            Self::UnknownVertex(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::geometry::shapes::{Pnt2, Polygon, Seg2, Tri2};

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
        expected.insert(Seg2::new(a, b));
        expected.insert(Seg2::new(b, d));
        expected.insert(Seg2::new(d, a));

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
        let result = polygon.triangles().remove(triangle);

        assert!(result.unwrap_err().is_unknown_vertex());
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
    fn remove_should_handle_triangle_with_hole() {
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
        expected.insert(Seg2::new(a, b));
        expected.insert(Seg2::new(b, c));
        expected.insert(Seg2::new(c, d));
        expected.insert(Seg2::new(d, x));
        expected.insert(Seg2::new(x, y));
        expected.insert(Seg2::new(y, z));
        expected.insert(Seg2::new(z, w));
        expected.insert(Seg2::new(w, x));
        expected.insert(Seg2::new(x, a));

        assert_eq!(polygon.edges(), &expected);
    }

    // TASK: Enable test. This will be easily possible once we can check whether
    //       a polygon line is outside the polygon. See commented code in
    //       `Triangles::remove` above.
    #[test]
    #[ignore]
    fn remove_should_recognize_that_triangle_is_in_hole() {
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

        let result = polygon.triangles().remove(Tri2::new(x, w, y));
        assert!(result.unwrap_err().is_outside_of_polygon());
    }
}
