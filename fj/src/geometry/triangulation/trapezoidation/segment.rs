use super::point::{self, Point};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Segment {
    lower: Point,
    upper: Point,
}

impl Segment {
    /// Form a segment from two points
    ///
    /// The points must have a defined relation to each other. If they don't,
    /// this constructor returns `None`.
    pub fn new(a: Point, b: Point) -> Option<Segment> {
        let a_to_b = a.relation_to(&b)?;

        let (lower, upper) = match a_to_b {
            point::Relation::Below => (a, b),
            point::Relation::Above => (b, a),
        };

        Some(Self { lower, upper })
    }

    pub fn lower(&self) -> Point {
        self.lower
    }

    pub fn upper(&self) -> Point {
        self.upper
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::triangulation::trapezoidation::point::Point;

    use super::Segment;

    #[test]
    fn segment_should_require_clear_relation_between_points() {
        let point = Point::new(0.0, 0.0);

        let segment = Segment::new(point, point);

        assert_eq!(segment, None);
    }

    #[test]
    fn segment_should_return_upper_and_lower_point() {
        let upper = Point::new(0.0, 1.0);
        let lower = Point::new(0.0, 0.0);

        let a = Segment::new(upper, lower).unwrap();
        let b = Segment::new(lower, upper).unwrap();

        assert_eq!(a.upper(), upper);
        assert_eq!(b.upper(), upper);
        assert_eq!(a.lower(), lower);
        assert_eq!(b.lower(), lower);
    }
}
