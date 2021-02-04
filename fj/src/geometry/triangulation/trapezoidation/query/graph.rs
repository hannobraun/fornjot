//! Defines the point location query structure

use std::collections::HashMap;

use crate::geometry::triangulation::trapezoidation::{
    point::Point, segment::Segment,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Graph<XNode = X, YNode = Y, Sink = Region> {
    nodes: HashMap<Id, Node<XNode, YNode, Sink>>,
    next_id: u64,
}

impl<XNode, YNode, Sink> Graph<XNode, YNode, Sink> {
    /// Construct a new `Graph` instance
    ///
    /// The graph initially contains single source/sink node.
    pub fn new() -> Self
    where
        Sink: Default,
    {
        let mut nodes = HashMap::new();
        nodes.insert(Id(0), Node::Sink(Sink::default()));

        Self { nodes, next_id: 1 }
    }

    pub fn source(&self) -> Id {
        Id(0)
    }

    pub fn get(&self, id: Id) -> &Node<XNode, YNode, Sink> {
        // The graph is append-only, so we know that every id that exists must
        // point to a valid node.
        self.nodes.get(&id).unwrap()
    }

    pub fn insert_sink(&mut self, sink: Sink) -> Id {
        let id = self.next_id;
        self.next_id += 1;

        self.nodes.insert(Id(id), Node::Sink(sink));

        Id(id)
    }

    pub fn replace(&mut self, id: Id, node: Node<XNode, YNode, Sink>) {
        self.nodes.insert(id, node);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Id(u64);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Node<XNode = X, YNode = Y, Sink = Region> {
    X(XNode),
    Y(YNode),
    Sink(Sink),
}

impl<XNode, YNode, Sink> Node<XNode, YNode, Sink> {
    pub fn is_sink(&self) -> bool {
        matches!(self, &Node::Sink(_))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct X {
    pub segment: Segment,
    pub left: Id,
    pub right: Id,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Y {
    pub point: Point,
    pub below: Id,
    pub above: Id,
}

#[derive(Debug, Default, PartialEq)]
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
    use super::Node;

    type Graph = super::Graph<X, Y, Sink>;

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    struct X(u64);

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    struct Y(u64);

    #[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
    struct Sink(u64);

    #[test]
    fn graph_should_be_constructed_with_root_node() {
        let graph = Graph::new();

        let root = graph.get(graph.source());
        assert_eq!(root, &Node::Sink(Sink(0)));
    }

    #[test]
    fn graph_should_insert_sinks() {
        let mut graph = Graph::new();

        let a = Sink(1);
        let b = Sink(2);

        let id_a = graph.insert_sink(a);
        let id_b = graph.insert_sink(b);

        assert_eq!(graph.get(id_a), &Node::Sink(a));
        assert_eq!(graph.get(id_b), &Node::Sink(b));
    }

    #[test]
    fn graph_should_replace_nodes() {
        let mut graph = Graph::new();

        let node = Node::X(X(1));
        graph.replace(graph.source(), node);

        assert_eq!(graph.get(graph.source()), &node);
    }
}
