//! Defines the point location query structure

use std::collections::HashMap;

use crate::geometry::triangulation::trapezoidation::{
    point::Point, segment::Segment,
};

// TASK: Add single source on construction.
// TASK: Implement behavior, as required by insertion and query code.
pub struct Graph(HashMap<Id, Node>);

pub struct Id(u32);

pub enum Node {
    NonSink(NonSink),
    Sink(Sink),
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

pub struct Sink;
