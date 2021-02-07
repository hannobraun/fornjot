use crate::geometry::triangulation::trapezoidation::point::Point;

use super::ids::Id;

#[derive(Debug, Default, PartialEq)]
pub struct Region {
    pub upper_boundary: Option<HorizontalBoundary>,
    pub lower_boundary: Option<HorizontalBoundary>,
    pub left_segment: Option<Id>,
    pub right_segment: Option<Id>,
}

impl RegionExt for Region {
    fn source() -> Self {
        Self {
            lower_boundary: None,
            upper_boundary: None,
            left_segment: None,
            right_segment: None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HorizontalBoundary {
    pub point: Point,
    pub regions: BoundingRegions,
}

#[derive(Debug, PartialEq)]
pub enum BoundingRegions {
    One(Id),
    Two { left: Id, right: Id },
}

/// Used by various unit test suites
#[cfg(test)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TestRegion(pub u64);

#[cfg(test)]
impl RegionExt for TestRegion {
    fn source() -> Self {
        Self(0)
    }
}

pub trait RegionExt {
    fn source() -> Self;

    // TASK: Add `split_x`.
    // TASK: Add `split_y`.
}

// TASK: Un-derive `Default` from region types.
// TASK: Make sure that insert method split regions.
