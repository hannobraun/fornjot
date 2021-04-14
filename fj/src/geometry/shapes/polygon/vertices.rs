use std::collections::HashSet;

use crate::geometry::shapes::Pnt2;

use super::data::PolygonData;

pub struct Vertices<'r>(pub(super) &'r mut PolygonData);

impl Vertices<'_> {
    pub fn neighbors_of(&self, vertex: impl Into<Pnt2>) -> HashSet<Pnt2> {
        let vertex = vertex.into();

        let mut neighbors = HashSet::new();

        for edge in self.0.edges() {
            if edge.a == vertex {
                neighbors.insert(edge.b);
            }
            if edge.b == vertex {
                neighbors.insert(edge.a);
            }
        }

        neighbors
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::shapes::{Pnt2, Polygon};

    #[test]
    fn neighbors_of_should_return_neighbors_of_vertex() {
        let mut polygon = Polygon::new();

        let a = Pnt2::from_f32s(0.0, 0.0);
        let b = Pnt2::from_f32s(1.0, 0.0);
        let c = Pnt2::from_f32s(0.0, 1.0);
        polygon.insert_chain(&[a, b, c]);

        let neighbors_of_a = polygon.vertices().neighbors_of(a);
        let neighbors_of_b = polygon.vertices().neighbors_of(b);
        let neighbors_of_c = polygon.vertices().neighbors_of(c);

        assert!(neighbors_of_a.contains(&b));
        assert!(neighbors_of_a.contains(&c));

        assert!(neighbors_of_b.contains(&a));
        assert!(neighbors_of_b.contains(&c));

        assert!(neighbors_of_c.contains(&a));
        assert!(neighbors_of_c.contains(&b));
    }
}
