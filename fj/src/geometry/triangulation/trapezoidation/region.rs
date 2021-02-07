use crate::geometry::triangulation::trapezoidation::point::Point;

use super::ids::Id;

#[derive(Debug, PartialEq)]
pub struct Region {
    pub upper_boundary: Option<HorizontalBoundary>,
    pub lower_boundary: Option<HorizontalBoundary>,
    pub left_segment: Option<Id>,
    pub right_segment: Option<Id>,
}

impl Source for Region {
    fn source() -> Self {
        Self {
            lower_boundary: None,
            upper_boundary: None,
            left_segment: None,
            right_segment: None,
        }
    }
}

impl Split for Region {
    fn split_x(self) -> (Self, Self) {
        // TASK: Implement
        todo!()
    }

    fn split_y(self) -> (Self, Self) {
        // TASK: Implement
        todo!()
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TestRegion(pub u64);

#[cfg(test)]
impl Source for TestRegion {
    fn source() -> Self {
        Self(0)
    }
}

// TASK: Don't use this trait in insert methods.
pub trait Source {
    fn source() -> Self;
}

// TASK: Implement for region types.
pub trait Split: Sized {
    fn split_x(self) -> (Self, Self);
    fn split_y(self) -> (Self, Self);
}

// TASK: Un-derive `Default` from region types.
// TASK: Make sure that insert methods split regions.
