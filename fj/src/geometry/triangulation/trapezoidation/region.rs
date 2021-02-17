use std::fmt::Debug;

use crate::geometry::triangulation::trapezoidation::point::Point;

use super::{graph::Graph, ids::Id, segment::Segment};

#[derive(Clone, Debug, PartialEq)]
pub struct Region {
    pub upper_boundary: Option<HorizontalBoundary>,
    pub lower_boundary: Option<HorizontalBoundary>,
    pub left_segment: Option<Segment>,
    pub right_segment: Option<Segment>,
}

impl FromId for Region {}

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
        (self.clone(), self.clone())
    }

    fn split_y(&self) -> (Self, Self) {
        (self.clone(), self.clone())
    }

    // `split_x` and `split_y` are the same right now, because I decided to keep
    // all intelligence out of them and instead in the updating logic. Maybe
    // they need to be merged (or replaced with `clone`).
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

impl BoundingRegions {
    pub fn iter(&self) -> Vec<Id> {
        let mut ids = Vec::new();

        match *self {
            Self::One(id) => ids.push(id),
            Self::Two { left, right } => {
                ids.push(left);
                ids.push(right);
            }
        }

        ids
    }

    pub fn replace(&mut self, old: Id, new: Id) {
        match self {
            Self::One(id) => {
                if *id == old {
                    *id = new;
                }
            }
            Self::Two { left, right } => {
                if *left == old {
                    *left = new;
                }
                if *right == old {
                    *right = new;
                }
            }
        }
    }
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
impl FromId for TestRegion {}

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

pub trait FromId: Sized + Debug {
    fn from_id<X, Y>(id: Id, graph: &Graph<X, Y, Self>) -> &Self
    where
        X: Debug,
        Y: Debug,
    {
        graph.get(id).sink().as_ref().unwrap()
    }
}

pub trait Source {
    fn source() -> Self;
}

pub trait Split: Sized {
    fn split_x(&self) -> (Self, Self);
    fn split_y(&self) -> (Self, Self);
}
