//! Intersection between a ray and a line segment in 2D

use fj_math::LineSegment;

use super::{HorizontalRayToTheRight, Intersect};

impl Intersect for (&HorizontalRayToTheRight<2>, &LineSegment<2>) {
    type Intersection = RaySegmentIntersection;

    fn intersect(self) -> Option<Self::Intersection> {
        let (ray, segment) = self;

        let [a, b] = segment.points;
        let [lower, upper] = if a.v <= b.v { [a, b] } else { [b, a] };
        let [left, right] = if a.u <= b.u { [a, b] } else { [b, a] };

        if ray.origin.v > upper.v {
            // ray is above segment
            return None;
        }
        if ray.origin.v < lower.v {
            // ray is below segment
            return None;
        }

        if ray.origin.v == lower.v && lower.v == upper.v {
            // ray and segment are parallel and at same height

            if ray.origin.u > right.u {
                return None;
            }

            if ray.origin.u == a.u {
                return Some(RaySegmentIntersection::RayStartsOnOnFirstVertex);
            }
            if ray.origin.u == b.u {
                return Some(RaySegmentIntersection::RayStartsOnSecondVertex);
            }
            if ray.origin.u > left.u && ray.origin.u < right.u {
                return Some(RaySegmentIntersection::RayStartsOnSegment);
            }

            return Some(RaySegmentIntersection::RayHitsSegmentAndAreParallel);
        }

        let pa = robust::Coord {
            x: lower.u,
            y: lower.v,
        };
        let pb = robust::Coord {
            x: upper.u,
            y: upper.v,
        };
        let pc = robust::Coord {
            x: ray.origin.u,
            y: ray.origin.v,
        };

        let orient2d = robust::orient2d(pa, pb, pc);

        if orient2d == 0. {
            // ray starts on the line

            if ray.origin.v == a.v {
                return Some(RaySegmentIntersection::RayStartsOnOnFirstVertex);
            }
            if ray.origin.v == b.v {
                return Some(RaySegmentIntersection::RayStartsOnSecondVertex);
            }

            return Some(RaySegmentIntersection::RayStartsOnSegment);
        }

        if orient2d > 0. {
            // ray starts left of the line

            if ray.origin.v == upper.v {
                return Some(RaySegmentIntersection::RayHitsUpperVertex);
            }
            if ray.origin.v == lower.v {
                return Some(RaySegmentIntersection::RayHitsLowerVertex);
            }

            return Some(RaySegmentIntersection::RayHitsSegment);
        }

        None
    }
}

/// An intersection between a ray and a line segment
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RaySegmentIntersection {
    /// The ray hit the segment itself
    RayHitsSegment,

    /// The ray hit the lower vertex of the segment
    RayHitsLowerVertex,

    /// The ray hit the upper vertex of the segment
    RayHitsUpperVertex,

    /// The ray hit the whole segment, as it is parallel to the ray
    RayHitsSegmentAndAreParallel,

    /// The ray starts on the segment
    RayStartsOnSegment,

    /// The ray starts on the first vertex of the segment
    RayStartsOnOnFirstVertex,

    /// The ray starts on the second vertex of the segment
    RayStartsOnSecondVertex,
}

#[cfg(test)]
mod tests {
    use fj_math::LineSegment;

    use crate::algorithms::intersect::Intersect;

    use super::{HorizontalRayToTheRight, RaySegmentIntersection};

    #[test]
    fn ray_is_left_of_segment() {
        let ray = HorizontalRayToTheRight::from_point([0., 2.]);

        let below = LineSegment::from([[1., 0.], [1., 1.]]);
        let above = LineSegment::from([[1., 3.], [1., 4.]]);
        let same_level = LineSegment::from([[1., 1.], [1., 3.]]);

        assert!((&ray, &below).intersect().is_none());
        assert!((&ray, &above).intersect().is_none());
        assert!(matches!(
            (&ray, &same_level).intersect(),
            Some(RaySegmentIntersection::RayHitsSegment)
        ));
    }

    #[test]
    fn ray_is_right_of_segment() {
        let ray = HorizontalRayToTheRight::from_point([1., 2.]);

        let same_level = LineSegment::from([[0., 1.], [0., 3.]]);
        assert!((&ray, &same_level).intersect().is_none());
    }

    #[test]
    fn ray_overlaps_with_segment_along_x_axis() {
        let ray = HorizontalRayToTheRight::from_point([1., 1.]);

        let no_hit = LineSegment::from([[0., 0.], [2., 3.]]);

        let hit_segment = LineSegment::from([[0., 0.], [3., 2.]]);
        let hit_upper = LineSegment::from([[0., 0.], [2., 1.]]);
        let hit_lower = LineSegment::from([[0., 2.], [2., 1.]]);

        assert!((&ray, &no_hit).intersect().is_none());
        assert!(matches!(
            (&ray, &hit_segment).intersect(),
            Some(RaySegmentIntersection::RayHitsSegment)
        ));
        assert!(matches!(
            (&ray, &hit_upper).intersect(),
            Some(RaySegmentIntersection::RayHitsUpperVertex),
        ));
        assert!(matches!(
            (&ray, &hit_lower).intersect(),
            Some(RaySegmentIntersection::RayHitsLowerVertex),
        ));
    }

    #[test]
    fn ray_starts_on_segment() {
        let ray = HorizontalRayToTheRight::from_point([1., 1.]);

        let hit_segment = LineSegment::from([[0., 0.], [2., 2.]]);
        let hit_upper = LineSegment::from([[0., 0.], [1., 1.]]);
        let hit_lower = LineSegment::from([[1., 1.], [2., 2.]]);

        assert!(matches!(
            (&ray, &hit_segment).intersect(),
            Some(RaySegmentIntersection::RayStartsOnSegment)
        ));
        assert!(matches!(
            (&ray, &hit_upper).intersect(),
            Some(RaySegmentIntersection::RayStartsOnSecondVertex),
        ));
        assert!(matches!(
            (&ray, &hit_lower).intersect(),
            Some(RaySegmentIntersection::RayStartsOnOnFirstVertex),
        ));
    }

    #[test]
    fn ray_and_segment_are_parallel_and_on_same_level() {
        let ray = HorizontalRayToTheRight::from_point([2., 0.]);

        let left = LineSegment::from([[0., 0.], [1., 0.]]);
        let right = LineSegment::from([[3., 0.], [4., 0.]]);

        assert!((&ray, &left).intersect().is_none());
        assert!(matches!(
            (&ray, &right).intersect(),
            Some(RaySegmentIntersection::RayHitsSegmentAndAreParallel)
        ));
    }

    #[test]
    fn ray_starts_on_parallel_segment() {
        let ray = HorizontalRayToTheRight::from_point([2., 0.]);

        let left = LineSegment::from([[0., 0.], [2., 0.]]);
        let overlapping = LineSegment::from([[1., 0.], [3., 0.]]);
        let right = LineSegment::from([[2., 0.], [4., 0.]]);

        assert!(matches!(
            (&ray, &left).intersect(),
            Some(RaySegmentIntersection::RayStartsOnSecondVertex)
        ));
        assert!(matches!(
            (&ray, &overlapping).intersect(),
            Some(RaySegmentIntersection::RayStartsOnSegment),
        ));
        assert!(matches!(
            (&ray, &right).intersect(),
            Some(RaySegmentIntersection::RayStartsOnOnFirstVertex),
        ));
    }
}
