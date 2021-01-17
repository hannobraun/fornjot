use nalgebra::Point2;

use super::Relation;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex(pub Point2<f32>);

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Point2::new(x, y))
    }

    pub fn relation_to(&self, other: &Vertex) -> Option<Relation> {
        // Whether a vertex is above or below another is the primary criterion.
        if self.0.y > other.0.y {
            return Some(Relation::AboveOrLeftOf);
        }
        if self.0.y < other.0.y {
            return Some(Relation::BelowOrRightOf);
        }

        // If y coordinates are equal, we look at the left-right relation.
        if self.0.y == other.0.y {
            if self.0.x < other.0.x {
                return Some(Relation::AboveOrLeftOf);
            }
            if self.0.x > other.0.x {
                return Some(Relation::BelowOrRightOf);
            }
        }

        // If we land here, the vertices are either equal, or we have NaN's or
        // some other weirdness.
        None
    }
}

impl From<Point2<f32>> for Vertex {
    fn from(point: Point2<f32>) -> Self {
        Vertex(point)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::trapezoidation::Relation;

    use super::Vertex;

    #[test]
    fn vertex_with_high_y_should_be_higher_than_vertex_with_low_y() {
        let upper = Vertex::new(0.0, 1.0);
        let lower = Vertex::new(0.0, 0.0);

        assert_eq!(upper.relation_to(&lower), Some(Relation::AboveOrLeftOf));
        assert_eq!(lower.relation_to(&upper), Some(Relation::BelowOrRightOf));
    }

    #[test]
    fn vertex_with_equal_y_but_left_x_should_be_higher_than_right_x() {
        let upper = Vertex::new(0.0, 0.0);
        let lower = Vertex::new(1.0, 0.0);

        assert_eq!(upper.relation_to(&lower), Some(Relation::AboveOrLeftOf));
        assert_eq!(lower.relation_to(&upper), Some(Relation::BelowOrRightOf));
    }

    #[test]
    fn vertex_should_not_be_higher_or_lower_than_equal_vertex() {
        let vertex = Vertex::new(0.0, 0.0);

        assert_eq!(vertex.relation_to(&vertex), None);
    }
}
