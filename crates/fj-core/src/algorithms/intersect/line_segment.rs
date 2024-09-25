use fj_math::{Aabb, LineSegment, Point, Scalar, Vector};

use crate::geometry::curves::line::Line;

/// An intersection between a [`Line`] and a [`Segment`]
#[derive(Debug, Eq, PartialEq)]
pub enum LineSegmentIntersection {
    /// Line and segment intersect at a point
    Point {
        /// The intersection point, given as a coordinate on the line
        point_on_line: Point<1>,
    },

    /// Line and segment are coincident
    Coincident {
        /// The end points of the segment, given as coordinates on the line
        points_on_line: [Point<1>; 2],
    },
}

impl LineSegmentIntersection {
    /// Determine the intersection between a [`Line`] and a [`Segment`]
    pub fn compute(line: &Line<2>, segment: &LineSegment<2>) -> Option<Self> {
        // Algorithm adapted from Real-Time Collision Detection by Christer
        // Ericson. See section 5.1.9.1, 2D Segment Intersection.

        let [a, b] = segment.points;

        // Find vector that is orthogonal to `segment`.
        let n = {
            let ab = b - a;
            Vector::from([ab.v, ab.u])
        };

        let n_dot_origin = n.dot(&(b - line.origin()));
        let n_dot_direction = n.dot(&line.direction());

        if n_dot_direction == Scalar::ZERO {
            // `line` and `segment` are parallel

            if n_dot_origin == Scalar::ZERO {
                // `line` and `segment` are not just parallel, but coincident!
                return Some(Self::Coincident {
                    points_on_line: segment
                        .points
                        .map(|point| line.point_to_line_coords(point)),
                });
            }

            return None;
        }

        // Now we ruled out the special cases. Compute where `line` hits the
        // line defined by `segment`'s points.
        let t = n_dot_origin / n_dot_direction;

        let point_is_on_segment = Aabb::<2>::from_points(segment.points)
            .contains(line.point_from_line_coords([t]));
        if !point_is_on_segment {
            return None;
        }

        Some(Self::Point {
            point_on_line: Point::from([t]),
        })
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{LineSegment, Point, Scalar, Vector};

    use crate::geometry::curves::line::Line;

    use super::LineSegmentIntersection;

    #[test]
    fn compute_one_hit() {
        let line =
            Line::from_origin_and_direction(Point::origin(), Vector::unit_u());

        assert_eq!(
            LineSegmentIntersection::compute(
                &line,
                &LineSegment::from_points([[1., -1.], [1., 1.]]),
            ),
            Some(LineSegmentIntersection::Point {
                point_on_line: Point::from([Scalar::ONE])
            }),
        );
    }

    #[test]
    fn compute_coincident() {
        let line =
            Line::from_origin_and_direction(Point::origin(), Vector::unit_u());

        assert_eq!(
            LineSegmentIntersection::compute(
                &line,
                &LineSegment::from_points([[1., 0.], [2., 0.]]),
            ),
            Some(LineSegmentIntersection::Coincident {
                points_on_line: [Point::from([1.]), Point::from([2.])],
            }),
        );
    }

    #[test]
    fn compute_no_hit_above() {
        let line =
            Line::from_origin_and_direction(Point::origin(), Vector::unit_u());

        assert_eq!(
            LineSegmentIntersection::compute(
                &line,
                &LineSegment::from_points([[1., 1.], [1., 2.]]),
            ),
            None,
        );
    }

    #[test]
    fn compute_no_hit_below() {
        let line =
            Line::from_origin_and_direction(Point::origin(), Vector::unit_u());

        assert_eq!(
            LineSegmentIntersection::compute(
                &line,
                &LineSegment::from_points([[1., -2.], [1., -1.]]),
            ),
            None,
        );
    }

    #[test]
    fn compute_no_hit_parallel() {
        let line =
            Line::from_origin_and_direction(Point::origin(), Vector::unit_u());

        assert_eq!(
            LineSegmentIntersection::compute(
                &line,
                &LineSegment::from_points([[-1., 1.], [1., 1.]]),
            ),
            None,
        );
    }
}
