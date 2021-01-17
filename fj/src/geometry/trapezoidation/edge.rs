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
}

#[cfg(test)]
mod tests {
    use crate::geometry::trapezoidation::Vertex;

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
}
