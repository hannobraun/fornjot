use crate::geometry::triangulation::trapezoidation::point::Point;

use super::ids::Id;

#[derive(Clone, Debug, PartialEq)]
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
    fn split_x(&self) -> (Self, Self) {
        // TASK: Implement
        todo!()
    }

    fn split_y(&self) -> (Self, Self) {
        // TASK: Implement
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct HorizontalBoundary {
    pub point: Point,
    pub regions: BoundingRegions,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BoundingRegions {
    One(Id),
    Two { left: Id, right: Id },
}

/// Used by various unit test suites
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TestRegion {
    pub id: u64,

    pub split_left: bool,
    pub split_right: bool,
    pub split_lower: bool,
    pub split_upper: bool,
}

#[cfg(test)]
impl TestRegion {
    pub fn new(id: u64) -> Self {
        Self {
            id,

            split_left: false,
            split_right: false,
            split_lower: false,
            split_upper: false,
        }
    }
}

#[cfg(test)]
impl Source for TestRegion {
    fn source() -> Self {
        Self::new(0)
    }
}

#[cfg(test)]
impl Split for TestRegion {
    fn split_x(&self) -> (Self, Self) {
        let left = Self {
            split_left: true,
            ..Self::new(self.id)
        };
        let right = Self {
            split_right: true,
            ..Self::new(self.id)
        };
        (left, right)
    }

    fn split_y(&self) -> (Self, Self) {
        let lower = Self {
            split_lower: true,
            ..Self::new(self.id)
        };
        let upper = Self {
            split_upper: true,
            ..Self::new(self.id)
        };
        (lower, upper)
    }
}

pub trait Source {
    fn source() -> Self;
}

pub trait Split: Sized {
    fn split_x(&self) -> (Self, Self);
    fn split_y(&self) -> (Self, Self);
}
