use nalgebra::Point2;

/// A point in the trapezoidation
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point(pub Point2<f32>);

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Point2::new(x, y))
    }

    pub fn x(&self) -> f32 {
        self.0.x
    }

    pub fn y(&self) -> f32 {
        self.0.y
    }

    /// Determine relation of this point to another
    ///
    /// Returns `None`, if the points have no relation to each other (because
    /// they are equal or something is NaN). Returns the relation otherwise.
    pub fn relation_to(&self, other: &Point) -> Option<Relation> {
        // Relation is primarily defined by the y-coordinate.
        if self.0.y < other.0.y {
            return Some(Relation::Below);
        }
        if self.0.y > other.0.y {
            return Some(Relation::Above);
        }

        // If y-coordinates are equal, we look at the x-coordinate.
        if self.0.y == other.0.y {
            if self.0.x < other.0.x {
                return Some(Relation::Below);
            }
            if self.0.x > other.0.x {
                return Some(Relation::Above);
            }
        }

        // If we land here, the points are either equal, or we have NaN's or
        // some other weirdness.
        None
    }
}

impl From<Point2<f32>> for Point {
    fn from(point: Point2<f32>) -> Self {
        Point(point)
    }
}

/// The relation between two points
///
/// Points in the trapezoidation must have a clear above-below relation to each
/// other. Since we can't rely on that with points from real polygons, we use
/// the lexicographic technique suggested in the paper: If two points are at the
/// same height, the one with the smaller x-coordinate is considered lower.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Relation {
    Below,
    Above,
}

#[cfg(test)]
mod tests {
    use super::{Point, Relation};

    #[test]
    fn vertex_with_high_y_should_be_higher_than_vertex_with_low_y() {
        let lower = Point::new(0.0, 0.0);
        let upper = Point::new(0.0, 1.0);

        assert_eq!(lower.relation_to(&upper), Some(Relation::Below));
        assert_eq!(upper.relation_to(&lower), Some(Relation::Above));
    }

    #[test]
    fn vertex_with_equal_y_but_larger_x_should_be_higher_than_lower_x() {
        let lower = Point::new(0.0, 0.0);
        let upper = Point::new(1.0, 0.0);

        assert_eq!(lower.relation_to(&upper), Some(Relation::Below));
        assert_eq!(upper.relation_to(&lower), Some(Relation::Above));
    }

    #[test]
    fn vertex_should_not_be_higher_or_lower_than_equal_vertex() {
        let vertex = Point::new(0.0, 0.0);

        assert_eq!(vertex.relation_to(&vertex), None);
    }
}
