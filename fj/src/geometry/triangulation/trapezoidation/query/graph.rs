//! Defines the point location query structure

use std::collections::HashMap;

use crate::geometry::triangulation::trapezoidation::{
    point::Point, segment::Segment,
};

// TASK: Implement behavior, as required by insertion and query code.
pub struct Graph {
    nodes: HashMap<Id, Node>,
}

impl Graph {
    /// Construct a new `Graph` instance
    ///
    /// The graph initially contains single source/sink node.
    pub fn new() -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(Id(0), Node::Sink(Region::new()));

        Self { nodes }
    }

    pub fn source(&self) -> Id {
        Id(0)
    }

    pub fn get(&self, id: Id) -> &Node {
        // The graph is append-only, so we know that every id that exists must
        // point to a valid node.
        self.nodes.get(&id).unwrap()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Id(u32);

#[derive(Debug, PartialEq)]
pub enum Node {
    NonSink(NonSink),
    Sink(Region),
}

#[derive(Debug, PartialEq)]
pub enum NonSink {
    X(X),
    Y(Y),
}

#[derive(Debug, PartialEq)]
pub struct X {
    pub segment: Segment,
    pub left: Id,
    pub right: Id,
}

#[derive(Debug, PartialEq)]
pub struct Y {
    pub point: Point,
    pub below: Id,
    pub above: Id,
}

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::{Graph, Node, Region};

    #[test]
    fn graph_should_be_constructed_with_root_node() {
        let graph = Graph::new();

        let root = graph.get(graph.source());
        assert_eq!(root, &Node::Sink(Region::new()));
    }
}
