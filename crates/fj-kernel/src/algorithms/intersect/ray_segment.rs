//! Intersection between a ray and a line segment in 2D

use fj_math::Segment;

use super::{HorizontalRayToTheRight, Intersect};

impl Intersect for (&HorizontalRayToTheRight<2>, &Segment<2>) {
    type Intersection = RaySegmentIntersection;

    fn intersect(self) -> Option<Self::Intersection> {
        let (ray, segment) = self;

        let [a, b] = segment.points();
        let [lower, upper] = if a.v <= b.v { [a, b] } else { [b, a] };
        let right = if a.u > b.u { a } else { b };

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

            return Some(RaySegmentIntersection::Parallel);
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

        if robust::orient2d(pa, pb, pc) >= 0. {
            // ray starts on the line or left of it

            if ray.origin.v == upper.v {
                return Some(RaySegmentIntersection::UpperVertex);
            }
            if ray.origin.v == lower.v {
                return Some(RaySegmentIntersection::LowerVertex);
            }

            return Some(RaySegmentIntersection::Segment);
        }

        None
    }
}

/// An intersection between a ray and a line segment
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RaySegmentIntersection {
    /// The ray hit the segment itself
    Segment,

    /// The ray hit the lower vertex of the segment
    LowerVertex,

    /// The ray hit the upper vertex of the segment
    UpperVertex,

    /// The ray hit the whole segment, as it is parallel to the ray
    Parallel,
}

#[cfg(test)]
mod tests {
    use fj_math::Segment;

    use crate::algorithms::intersect::Intersect;

    use super::{HorizontalRayToTheRight, RaySegmentIntersection};

    #[test]
    fn ray_is_left_of_segment() {
        let ray = HorizontalRayToTheRight::from([0., 2.]);

        let below = Segment::from([[1., 0.], [1., 1.]]);
        let above = Segment::from([[1., 3.], [1., 4.]]);
        let same_level = Segment::from([[1., 1.], [1., 3.]]);

        assert!((&ray, &below).intersect().is_none());
        assert!((&ray, &above).intersect().is_none());
        assert!(matches!(
            (&ray, &same_level).intersect(),
            Some(RaySegmentIntersection::Segment)
        ));
    }

    #[test]
    fn ray_is_right_of_segment() {
        let ray = HorizontalRayToTheRight::from([1., 2.]);

        let same_level = Segment::from([[0., 1.], [0., 3.]]);
        assert!((&ray, &same_level).intersect().is_none());
    }

    #[test]
    fn ray_overlaps_with_segment_along_x_axis() {
        let ray = HorizontalRayToTheRight::from([1., 1.]);

        let no_hit = Segment::from([[0., 0.], [2., 3.]]);

        let hit_segment = Segment::from([[0., 0.], [3., 2.]]);
        let hit_upper = Segment::from([[0., 0.], [2., 1.]]);
        let hit_lower = Segment::from([[0., 2.], [2., 1.]]);

        assert!((&ray, &no_hit).intersect().is_none());
        assert!(matches!(
            (&ray, &hit_segment).intersect(),
            Some(RaySegmentIntersection::Segment)
        ));
        assert!(matches!(
            (&ray, &hit_upper).intersect(),
            Some(RaySegmentIntersection::UpperVertex),
        ));
        assert!(matches!(
            (&ray, &hit_lower).intersect(),
            Some(RaySegmentIntersection::LowerVertex),
        ));
    }

    #[test]
    fn ray_starts_on_segment() {
        let ray = HorizontalRayToTheRight::from([1., 1.]);

        let hit_segment = Segment::from([[0., 0.], [2., 2.]]);
        let hit_upper = Segment::from([[0., 0.], [1., 1.]]);
        let hit_lower = Segment::from([[1., 1.], [2., 2.]]);

        assert!(matches!(
            (&ray, &hit_segment).intersect(),
            Some(RaySegmentIntersection::Segment)
        ));
        assert!(matches!(
            (&ray, &hit_upper).intersect(),
            Some(RaySegmentIntersection::UpperVertex),
        ));
        assert!(matches!(
            (&ray, &hit_lower).intersect(),
            Some(RaySegmentIntersection::LowerVertex),
        ));
    }

    #[test]
    fn ray_and_segment_are_parallel_and_on_same_level() {
        let ray = HorizontalRayToTheRight::from([2., 0.]);

        let left = Segment::from([[0., 0.], [1., 0.]]);
        let overlapping = Segment::from([[1., 0.], [3., 0.]]);
        let right = Segment::from([[3., 0.], [4., 0.]]);

        assert!((&ray, &left).intersect().is_none());
        assert!(matches!(
            (&ray, &overlapping).intersect(),
            Some(RaySegmentIntersection::Parallel)
        ));
        assert!(matches!(
            (&ray, &right).intersect(),
            Some(RaySegmentIntersection::Parallel)
        ));
    }
}
