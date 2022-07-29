use fj_math::{Aabb, Line, Scalar, Segment, Vector};

/// An intersection between a [`Line`] and a [`Segment`]
#[derive(Debug, Eq, PartialEq)]
pub enum LineSegmentIntersection {
    /// Line and segment intersect on a point
    ///
    /// Point is given as a coordinate on the line.
    PointOnLine(Scalar),

    /// Line and segment are coincident
    Coincident,
}

impl LineSegmentIntersection {
    /// Determine the intersection between a [`Line`] and a [`Segment`]
    pub fn line_segment(
        line: &Line<2>,
        segment: &Segment<2>,
    ) -> Option<LineSegmentIntersection> {
        // Algorithm adapted from Real-Time Collision Detection by Christer
        // Ericson. See section 5.1.9.1, 2D Segment Intersection.

        let [a, b] = segment.points();

        // Find vector that is orthogonal to `segment`.
        let n = {
            let ab = b - a;
            Vector::from([ab.v, ab.u])
        };

        let n_dot_origin = n.dot(&(b - line.origin));
        let n_dot_direction = n.dot(&line.direction);

        if n_dot_origin == Scalar::ZERO && n_dot_direction == Scalar::ZERO {
            // `line` and `segment` are not just parallel, but coincident!
            return Some(LineSegmentIntersection::Coincident);
        }

        if n_dot_direction == Scalar::ZERO {
            // `line` and `segment` are parallel, but not coincident
            return None;
        }

        // Now we ruled out the special cases. Compute where `line` hits the
        // line defined by `segment`'s points.
        let t = n_dot_origin / n_dot_direction;

        let point_is_on_segment = Aabb::<2>::from_points(segment.points())
            .contains(line.point_from_line_coords([t]));
        if !point_is_on_segment {
            return None;
        }

        Some(LineSegmentIntersection::PointOnLine(t))
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{Line, Point, Scalar, Segment, Vector};

    use crate::algorithms::intersection::LineSegmentIntersection;

    #[test]
    fn line_segment() {
        let line = Line {
            origin: Point::origin(),
            direction: Vector::unit_u(),
        };

        // regular hit
        assert_eq!(
            LineSegmentIntersection::line_segment(
                &line,
                &Segment::from_points([[1., -1.], [1., 1.]]),
            ),
            Some(LineSegmentIntersection::PointOnLine(Scalar::ONE)),
        );

        // hit, where line and segment are parallel
        assert_eq!(
            LineSegmentIntersection::line_segment(
                &line,
                &Segment::from_points([[1., 0.], [2., 0.]]),
            ),
            Some(LineSegmentIntersection::Coincident),
        );

        // segment above line
        assert_eq!(
            LineSegmentIntersection::line_segment(
                &line,
                &Segment::from_points([[1., 1.], [1., 2.]]),
            ),
            None,
        );

        // segment below line
        assert_eq!(
            LineSegmentIntersection::line_segment(
                &line,
                &Segment::from_points([[1., -2.], [1., -1.]]),
            ),
            None,
        );

        // segment parallel to line
        assert_eq!(
            LineSegmentIntersection::line_segment(
                &line,
                &Segment::from_points([[-1., 1.], [1., 1.]]),
            ),
            None,
        );
    }
}
