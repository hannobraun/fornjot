//! Ray casting

use fj_math::{Point, Segment};

/// Implemented by types that support ray casting
pub trait CastRay<const D: usize> {
    /// The type that describes a hit of the ray on the implementing type
    type Hit;

    /// Cast a ray against `self`
    fn cast_ray(&self, ray: HorizontalRayToTheRight<D>) -> Option<Self::Hit>;
}

impl CastRay<2> for Segment<2> {
    type Hit = RaySegmentHit;

    fn cast_ray(
        &self,
        ray: HorizontalRayToTheRight<2>,
    ) -> Option<RaySegmentHit> {
        let [a, b] = self.points();
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

            return Some(RaySegmentHit::Parallel);
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
                return Some(RaySegmentHit::UpperVertex);
            }
            if ray.origin.v == lower.v {
                return Some(RaySegmentHit::LowerVertex);
            }

            return Some(RaySegmentHit::Segment);
        }

        None
    }
}

/// A horizontal ray that goes to the right
///
/// For in-kernel use, we don't need anything more flexible, and being exactly
/// horizontal simplifies some calculations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HorizontalRayToTheRight<const D: usize> {
    /// The point where the ray originates
    pub origin: Point<D>,
}

impl<P, const D: usize> From<P> for HorizontalRayToTheRight<D>
where
    P: Into<Point<D>>,
{
    fn from(point: P) -> Self {
        Self {
            origin: point.into(),
        }
    }
}

/// A hit between a ray and a line segment
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RaySegmentHit {
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

    use crate::algorithms::cast_ray::CastRay;

    use super::{HorizontalRayToTheRight, RaySegmentHit};

    #[test]
    fn hits_segment_right() {
        let ray = HorizontalRayToTheRight::from([0., 2.]);

        let below = Segment::from([[1., 0.], [1., 1.]]);
        let above = Segment::from([[1., 3.], [1., 4.]]);
        let same_level = Segment::from([[1., 1.], [1., 3.]]);

        assert!(below.cast_ray(ray).is_none());
        assert!(above.cast_ray(ray).is_none());
        assert!(matches!(
            same_level.cast_ray(ray),
            Some(RaySegmentHit::Segment)
        ));
    }

    #[test]
    fn hits_segment_left() {
        let ray = HorizontalRayToTheRight::from([1., 2.]);

        let same_level = Segment::from([[0., 1.], [0., 3.]]);
        assert!(same_level.cast_ray(ray).is_none());
    }

    #[test]
    fn hits_segment_overlapping() {
        let ray = HorizontalRayToTheRight::from([1., 1.]);

        let no_hit = Segment::from([[0., 0.], [2., 3.]]);

        let hit_segment = Segment::from([[0., 0.], [3., 2.]]);
        let hit_upper = Segment::from([[0., 0.], [2., 1.]]);
        let hit_lower = Segment::from([[0., 2.], [2., 1.]]);

        assert!(no_hit.cast_ray(ray).is_none());
        assert!(matches!(
            hit_segment.cast_ray(ray),
            Some(RaySegmentHit::Segment)
        ));
        assert!(matches!(
            hit_upper.cast_ray(ray),
            Some(RaySegmentHit::UpperVertex),
        ));
        assert!(matches!(
            hit_lower.cast_ray(ray),
            Some(RaySegmentHit::LowerVertex),
        ));
    }

    #[test]
    fn hits_segment_on_segment() {
        let ray = HorizontalRayToTheRight::from([1., 1.]);

        let hit_segment = Segment::from([[0., 0.], [2., 2.]]);
        let hit_upper = Segment::from([[0., 0.], [1., 1.]]);
        let hit_lower = Segment::from([[1., 1.], [2., 2.]]);

        assert!(matches!(
            hit_segment.cast_ray(ray),
            Some(RaySegmentHit::Segment)
        ));
        assert!(matches!(
            hit_upper.cast_ray(ray),
            Some(RaySegmentHit::UpperVertex),
        ));
        assert!(matches!(
            hit_lower.cast_ray(ray),
            Some(RaySegmentHit::LowerVertex),
        ));
    }

    #[test]
    fn hits_segment_parallel() {
        let ray = HorizontalRayToTheRight::from([2., 0.]);

        let left = Segment::from([[0., 0.], [1., 0.]]);
        let overlapping = Segment::from([[1., 0.], [3., 0.]]);
        let right = Segment::from([[3., 0.], [4., 0.]]);

        assert!(left.cast_ray(ray).is_none());
        assert!(matches!(
            overlapping.cast_ray(ray),
            Some(RaySegmentHit::Parallel)
        ));
        assert!(matches!(right.cast_ray(ray), Some(RaySegmentHit::Parallel)));
    }
}
