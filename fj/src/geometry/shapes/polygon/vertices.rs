use std::collections::BTreeSet;

use crate::geometry::shapes::Point;

use super::data::PolygonData;

pub struct Vertices<'r>(pub(super) &'r mut PolygonData);

impl Vertices<'_> {
    pub fn iter(&self) -> impl Iterator<Item = Point<2>> + '_ {
        self.0.vertices()
    }

    pub fn neighbors_of(
        &self,
        vertex: impl Into<Point<2>>,
    ) -> BTreeSet<Point<2>> {
        let vertex = vertex.into();

        let mut neighbors = BTreeSet::new();

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
    use crate::geometry::shapes::{Point, Polygon};

    #[test]
    fn neighbors_of_should_return_neighbors_of_vertex() {
        let mut polygon = Polygon::new();

        let a = Point::new(0.0, 0.0);
        let b = Point::new(1.0, 0.0);
        let c = Point::new(0.0, 1.0);
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
