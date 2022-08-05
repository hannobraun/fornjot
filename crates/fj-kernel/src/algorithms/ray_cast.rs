//! Ray casting

use fj_math::{Point, Segment};

/// A horizontal ray that goes to the right
///
/// For in-kernel use, we don't need anything more flexible, and being exactly
/// horizontal simplifies some calculations.
pub struct HorizontalRayToTheRight<const D: usize> {
    /// The point where the ray originates
    pub origin: Point<D>,
}

impl HorizontalRayToTheRight<2> {
    /// Determine whether the ray hits the given line segment
    pub fn hits_segment(
        &self,
        segment: impl Into<Segment<2>>,
    ) -> Option<RaySegmentHit> {
        let [a, b] = segment.into().points();
        let [lower, upper] = if a.v <= b.v { [a, b] } else { [b, a] };
        let right = if a.u > b.u { a } else { b };

        if self.origin.v > upper.v {
            // ray is above segment
            return None;
        }
        if self.origin.v < lower.v {
            // ray is below segment
            return None;
        }

        if self.origin.v == lower.v && lower.v == upper.v {
            // ray and segment are parallel and at same height

            if self.origin.u > right.u {
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
            x: self.origin.u,
            y: self.origin.v,
        };

        if robust::orient2d(pa, pb, pc) >= 0. {
            // ray starts on the line or left of it

            if self.origin.v == upper.v {
                return Some(RaySegmentHit::UpperVertex);
            }
            if self.origin.v == lower.v {
                return Some(RaySegmentHit::LowerVertex);
            }

            return Some(RaySegmentHit::Segment);
        }

        None
    }
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
    use super::{HorizontalRayToTheRight, RaySegmentHit};

    #[test]
    fn hits_segment_right() {
        let ray = HorizontalRayToTheRight::from([0., 2.]);

        let below = [[1., 0.], [1., 1.]];
        let above = [[1., 3.], [1., 4.]];
        let same_level = [[1., 1.], [1., 3.]];

        assert!(ray.hits_segment(below).is_none());
        assert!(ray.hits_segment(above).is_none());
        assert!(matches!(
            ray.hits_segment(same_level),
            Some(RaySegmentHit::Segment)
        ));
    }

    #[test]
    fn hits_segment_left() {
        let ray = HorizontalRayToTheRight::from([1., 2.]);

        let same_level = [[0., 1.], [0., 3.]];
        assert!(ray.hits_segment(same_level).is_none());
    }

    #[test]
    fn hits_segment_overlapping() {
        let ray = HorizontalRayToTheRight::from([1., 1.]);

        let no_hit = [[0., 0.], [2., 3.]];

        let hit_segment = [[0., 0.], [3., 2.]];
        let hit_upper = [[0., 0.], [2., 1.]];
        let hit_lower = [[0., 2.], [2., 1.]];

        assert!(ray.hits_segment(no_hit).is_none());
        assert!(matches!(
            ray.hits_segment(hit_segment),
            Some(RaySegmentHit::Segment)
        ));
        assert!(matches!(
            ray.hits_segment(hit_upper),
            Some(RaySegmentHit::UpperVertex),
        ));
        assert!(matches!(
            ray.hits_segment(hit_lower),
            Some(RaySegmentHit::LowerVertex),
        ));
    }

    #[test]
    fn hits_segment_on_segment() {
        let ray = HorizontalRayToTheRight::from([1., 1.]);

        let hit_segment = [[0., 0.], [2., 2.]];
        let hit_upper = [[0., 0.], [1., 1.]];
        let hit_lower = [[1., 1.], [2., 2.]];

        assert!(matches!(
            ray.hits_segment(hit_segment),
            Some(RaySegmentHit::Segment)
        ));
        assert!(matches!(
            ray.hits_segment(hit_upper),
            Some(RaySegmentHit::UpperVertex),
        ));
        assert!(matches!(
            ray.hits_segment(hit_lower),
            Some(RaySegmentHit::LowerVertex),
        ));
    }

    #[test]
    fn hits_segment_parallel() {
        let ray = HorizontalRayToTheRight::from([2., 0.]);

        let left = [[0., 0.], [1., 0.]];
        let overlapping = [[1., 0.], [3., 0.]];
        let right = [[3., 0.], [4., 0.]];

        assert!(ray.hits_segment(left).is_none());
        assert!(matches!(
            ray.hits_segment(overlapping),
            Some(RaySegmentHit::Parallel)
        ));
        assert!(matches!(
            ray.hits_segment(right),
            Some(RaySegmentHit::Parallel)
        ));
    }
}
