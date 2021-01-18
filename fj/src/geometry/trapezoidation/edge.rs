use ncollide2d::{math::Isometry, query::PointQuery as _, shape::Segment};

use super::{Relation, Vertex};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    upper: Vertex,
    lower: Vertex,
}

impl Edge {
    pub fn new(a: Vertex, b: Vertex) -> Option<Self> {
        let (upper, lower) = match a.relation_to(&b) {
            Some(Relation::AboveOrLeftOf) => (a, b),
            Some(Relation::BelowOrRightOf) => (b, a),
            None => {
                // No clear relation between vertices. Probably because they're
                // equal.
                return None;
            }
        };

        Some(Self { upper, lower })
    }

    pub fn upper(&self) -> Vertex {
        self.upper
    }

    pub fn lower(&self) -> Vertex {
        self.lower
    }

    pub fn relation_to_vertex(&self, vertex: &Vertex) -> Option<Relation> {
        let this = Segment::new(self.upper().0, self.lower().0);
        let closest_point_on_edge = this
            .project_point(&Isometry::identity(), &vertex.0, false)
            .point;

        Vertex::from(closest_point_on_edge).relation_to(vertex)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::trapezoidation::{Relation, Vertex};

    use super::Edge;

    #[test]
    fn edge_should_require_clear_relation_between_vertices() {
        let vertex = Vertex::new(0.0, 0.0);

        let edge = Edge::new(vertex, vertex);

        assert_eq!(edge, None);
    }

    #[test]
    fn edge_should_return_upper_and_lower_vertex() {
        let upper = Vertex::new(0.0, 1.0);
        let lower = Vertex::new(0.0, 0.0);

        let a = Edge::new(upper, lower).unwrap();
        let b = Edge::new(lower, upper).unwrap();

        assert_eq!(a.upper(), upper);
        assert_eq!(b.upper(), upper);
        assert_eq!(a.lower(), lower);
        assert_eq!(b.lower(), lower);
    }

    #[test]
    fn edge_should_compute_relation_to_vertex() {
        let edge =
            Edge::new(Vertex::new(0.0, 2.0), Vertex::new(2.0, 0.0)).unwrap();

        let vertex_on_edge = Vertex::new(1.0, 1.0);
        let vertex_above_edge = Vertex::new(1.5, 1.5);
        let vertex_below_edge = Vertex::new(0.5, 0.5);

        assert_eq!(edge.relation_to_vertex(&vertex_on_edge), None);
        assert_eq!(
            edge.relation_to_vertex(&vertex_above_edge),
            Some(Relation::BelowOrRightOf)
        );
        assert_eq!(
            edge.relation_to_vertex(&vertex_below_edge),
            Some(Relation::AboveOrLeftOf)
        );
    }
}
