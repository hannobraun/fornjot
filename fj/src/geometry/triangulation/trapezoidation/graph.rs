//! Defines the point location query structure

use std::{collections::HashMap, fmt::Debug};

use crate::geometry::triangulation::trapezoidation::{
    point::Point, segment::Segment,
};

use super::{
    ids::{Id, Ids},
    region::{self, Region},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Graph<XNode = X, YNode = Y, Sink = Region> {
    ids: Ids,
    source: Id,
    nodes: HashMap<Id, Node<XNode, YNode, Sink>>,
}

impl<XNode, YNode, Sink> Graph<XNode, YNode, Sink> {
    /// Construct a new `Graph` instance
    ///
    /// The graph initially contains single source/sink node.
    pub fn new() -> Self
    where
        Sink: region::Source,
    {
        let mut ids = Ids::new();
        let source = ids.next();

        let mut nodes = HashMap::new();
        nodes.insert(source, Node::Sink(Sink::source()));

        Self { ids, source, nodes }
    }

    pub fn source(&self) -> Id {
        self.source
    }

    pub fn get(&self, id: Id) -> &Node<XNode, YNode, Sink> {
        // The graph is append-only, so we know that every id that exists must
        // point to a valid node.
        self.nodes.get(&id).unwrap()
    }

    pub fn get_mut(&mut self, id: Id) -> &mut Node<XNode, YNode, Sink> {
        // The graph is append-only, so we know that every id that exists must
        // point to a valid node.
        self.nodes.get_mut(&id).unwrap()
    }

    pub fn insert_sink(&mut self, sink: Sink) -> Id {
        let id = self.ids.next();
        self.nodes.insert(id, Node::Sink(sink));
        id
    }

    pub fn replace(&mut self, id: Id, node: Node<XNode, YNode, Sink>) {
        self.nodes.insert(id, node);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Node<XNode = X, YNode = Y, Sink = Region> {
    X(XNode),
    Y(YNode),
    Sink(Sink),
}

impl<XNode, YNode, Sink> Node<XNode, YNode, Sink> {
    pub fn x(&self) -> Option<&XNode> {
        match self {
            Node::X(x) => Some(x),
            _ => None,
        }
    }

    pub fn y(&self) -> Option<&YNode> {
        match self {
            Node::Y(y) => Some(y),
            _ => None,
        }
    }

    pub fn sink(&self) -> Option<&Sink> {
        match self {
            Node::Sink(sink) => Some(sink),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct X {
    pub segment: Segment,
    pub left: Id,
    pub right: Id,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Y {
    pub point: Point,
    pub below: Id,
    pub above: Id,
}

#[cfg(test)]
mod tests {
    use crate::geometry::triangulation::trapezoidation::region::TestRegion as Sink;

    use super::Node;

    type Graph = super::Graph<X, Y, Sink>;

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    struct X(u64);

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    struct Y(u64);

    #[test]
    fn graph_should_be_constructed_with_root_node() {
        let graph = Graph::new();

        let root = graph.get(graph.source());
        assert_eq!(root, &Node::Sink(Sink::new(0)));
    }

    #[test]
    fn graph_should_insert_sinks() {
        let mut graph = Graph::new();

        let a = Sink::new(1);
        let b = Sink::new(2);

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
