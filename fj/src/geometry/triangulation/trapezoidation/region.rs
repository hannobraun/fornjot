use crate::geometry::triangulation::trapezoidation::point::Point;

use super::ids::Id;

#[derive(Debug, Default, PartialEq)]
pub struct Region {
    pub upper_boundary: Option<HorizontalBoundary>,
    pub lower_boundary: Option<HorizontalBoundary>,
    pub left_segment: Option<Id>,
    pub right_segment: Option<Id>,
}

impl Region {
    pub fn new() -> Self {
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

// TASK: Add `RegionExt` trait that defines methods for splitting, and is
//       implemented for real and test region types.
// TASK: Un-derive `Default` from region types.
// TASK: Make sure that insert method split regions.
