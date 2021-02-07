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

// TASK: Consolidate the various test regions into a `TestRegion` that is
//       `#[cfg(test)]` and lives here.
// TASK: Add `RegionExt` trait that defines methods for splitting, and is
//       implemented for real and test region types.
// TASK: Move region code into separate module. Probably requires moving id code
//       into separate module first.
