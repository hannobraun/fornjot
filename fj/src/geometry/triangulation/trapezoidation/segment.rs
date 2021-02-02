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

    /// Compute the relation from a point to the segment
    ///
    /// The relation returned is from the perspective of the point: Left means
    /// the point is left of the segment, right means it's right of it.
    pub fn relation_from_point(&self, p: &Point) -> Option<Relation> {
        if p.relation_to(&self.lower) == Some(point::Relation::Below) {
            return None;
        }
        if p.relation_to(&self.upper) == Some(point::Relation::Above) {
            return None;
        }

        // Determine if point is left or right of the line defined by lower and
        // upper. The line is defined as going upwards, so left of the line
        // matches the intuitive definition of left in cartesian space.
        //
        // The formula is lifted from here:
        // https://math.stackexchange.com/a/274728
        let d = (p.x() - self.lower.x()) * (self.upper.y() - self.lower.y())
            - (p.y() - self.lower.y()) * (self.upper.x() - self.lower.x());

        if d == 0.0 {
            // Point is on segment.
            return None;
        }
        if d < 0.0 {
            return Some(Relation::Left);
        }
        if d > 0.0 {
            return Some(Relation::Right);
        }

        // We shouldn't ever reach this point, but if we do, there are probably
        // `NaN`s or other shenanigans going on.
        panic!("Invalid point ({:?}) or segment ({:?})");
    }

    // TASK: Implement `relation_to_point`.
    //       Returns `Option<point::Relation>`. Segment is below its upper
    //       point, above its lower point.
    // TASK: Implement `relation_to_segment`.
    //       Returns`Option<segment::Relation>`.
}

/// The relation between a point and a segment
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Relation {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use crate::geometry::triangulation::trapezoidation::{
        point::Point, segment,
    };

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

    #[test]
    fn segment_should_compute_relation_of_point_to_segment() {
        let segment =
            Segment::new(Point::new(0.0, 2.0), Point::new(2.0, 0.0)).unwrap();

        let point_on = Point::new(1.0, 1.0);
        let point_below = Point::new(0.0, -1.0);
        let point_above = Point::new(1.0, 3.0);

        assert_eq!(segment.relation_from_point(&point_on), None);
        assert_eq!(segment.relation_from_point(&point_below), None);
        assert_eq!(segment.relation_from_point(&point_above), None);

        let point_left = Point::new(0.5, 0.5);
        let point_right = Point::new(1.5, 1.5);

        assert_eq!(
            segment.relation_from_point(&point_left),
            Some(segment::Relation::Left)
        );
        assert_eq!(
            segment.relation_from_point(&point_right),
            Some(segment::Relation::Right)
        );
    }
}
