//! Defines the point location query structure

use std::collections::HashMap;

use crate::geometry::triangulation::trapezoidation::{
    point::Point, segment::Segment,
};

// TASK: Add single source on construction.
// TASK: Implement behavior, as required by insertion and query code.
pub struct Graph(HashMap<Id, Node>);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Id(u32);

pub enum Node {
    NonSink(NonSink),
    Sink(Region),
}

pub enum NonSink {
    X(X),
    Y(Y),
}

pub struct X {
    _segment: Segment,
    _left: Id,
    _right: Id,
}

pub struct Y {
    _point: Point,
    _below: Id,
    _above: Id,
}

pub struct Region {
    pub left_segment: Option<Id>,
    pub right_segment: Option<Id>,
    pub lower_left_region: Option<Id>,
    pub lower_right_region: Option<Id>,
    pub upper_left_region: Option<Id>,
    pub upper_right_region: Option<Id>,
}

impl Region {
    pub fn new() -> Self {
        Self {
            left_segment: None,
            right_segment: None,
            lower_left_region: None,
            lower_right_region: None,
            upper_left_region: None,
            upper_right_region: None,
        }
    }
}
