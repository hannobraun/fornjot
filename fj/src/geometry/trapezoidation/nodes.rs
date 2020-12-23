use std::collections::HashMap;

pub struct Nodes<Branch, Leaf> {
    pub map: HashMap<u32, Node<Branch, Leaf>>,
    pub next_id: u32,
}

impl<Branch, Leaf> Nodes<Branch, Leaf> {
    pub fn new() -> Self {
        Nodes {
            map: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn insert_leaf(&mut self, leaf: Leaf) -> Strong<LeafId> {
        let id = self.next_id;
        self.next_id += 1;

        self.map.insert(
            id,
            Node {
                parent: None,
                kind: NodeKind::Leaf(leaf),
            },
        );

        Strong(LeafId(NodeId(id)))
    }

    pub fn get(&self, id: impl Into<NodeId>) -> &Node<Branch, Leaf> {
        self.map.get(&id.into().0).unwrap()
    }

    pub fn get_mut(
        &mut self,
        id: impl Into<NodeId>,
    ) -> &mut Node<Branch, Leaf> {
        self.map.get_mut(&id.into().0).unwrap()
    }

    pub fn leafs(&self) -> impl Iterator<Item = (LeafId, &Leaf)> + '_ {
        self.map.iter().filter_map(|(&id, node)| match &node.kind {
            NodeKind::Leaf(leaf) => Some((LeafId(NodeId(id)), leaf)),
            _ => None,
        })
    }
}

/// Identifies a node
///
/// Since nodes can only be added, never removed, a `NodeId` instance is always
/// going to be valid.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct NodeId(pub u32);

impl From<Strong<NodeId>> for NodeId {
    fn from(strong: Strong<NodeId>) -> Self {
        strong.0
    }
}

impl From<LeafId> for NodeId {
    fn from(leaf_id: LeafId) -> Self {
        leaf_id.0
    }
}

impl From<Strong<LeafId>> for NodeId {
    fn from(strong: Strong<LeafId>) -> Self {
        strong.0.into()
    }
}

/// Identifies a leaf node
///
/// A more specific version of `NodeId`. Can be converted into a `NodeId`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LeafId(pub NodeId);

/// A strong version of a handle
///
/// Normal handles are `Copy`. `Strong` isn't, an attribute that is used by
/// `Nodes` to guarantee certain properties of nodes, for example that one node
/// can only ever be the child of one other node.
#[derive(Debug, PartialEq)]
pub struct Strong<T>(pub T);

impl Strong<NodeId> {
    pub fn as_node_id(&self) -> NodeId {
        self.0
    }
}

impl Strong<LeafId> {
    pub fn as_leaf_id(&self) -> LeafId {
        self.0
    }
}

impl From<Strong<LeafId>> for Strong<NodeId> {
    fn from(leaf_id: Strong<LeafId>) -> Self {
        Self(leaf_id.0.into())
    }
}

#[derive(Debug, PartialEq)]
pub struct Node<Branch, Leaf> {
    pub parent: Option<Strong<NodeId>>,
    pub kind: NodeKind<Branch, Leaf>,
}

#[derive(Debug, PartialEq)]
pub enum NodeKind<Branch, Leaf> {
    Branch(BranchNode<Branch>),
    Leaf(Leaf),
}

#[derive(Debug, PartialEq)]
pub struct BranchNode<T> {
    pub above: Strong<NodeId>,
    pub below: Strong<NodeId>,
    pub branch: T,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Relation {
    Above,
    Below,
}

#[cfg(test)]
mod tests {
    use super::{Node, NodeKind};

    type Nodes = super::Nodes<(), u8>;

    #[test]
    fn nodes_should_insert_leafs() {
        let mut nodes = Nodes::new();

        let leaf = 5;
        let id = nodes.insert_leaf(leaf);

        let mut expected_node = Node {
            parent: None,
            kind: NodeKind::Leaf(leaf),
        };

        assert_eq!(nodes.get(id.as_leaf_id()), &expected_node);
        assert_eq!(nodes.get_mut(id), &mut expected_node);
    }

    #[test]
    fn nodes_should_assign_new_id_when_adding_nodes() {
        let mut nodes = Nodes::new();

        let id_a = nodes.insert_leaf(5);
        let id_b = nodes.insert_leaf(8);

        assert_ne!(id_a, id_b);
    }

    #[test]
    fn nodes_should_return_all_leafs() {
        let mut nodes = Nodes::new();

        let leaf_a = 5;
        let leaf_b = 8;

        let id_a = nodes.insert_leaf(leaf_a);
        let id_b = nodes.insert_leaf(leaf_b);

        let mut saw_a = false;
        let mut saw_b = false;

        for (id, leaf) in nodes.leafs() {
            if id == id_a.as_leaf_id() && leaf == &leaf_a {
                saw_a = true;
            }
            if id == id_b.as_leaf_id() && leaf == &leaf_b {
                saw_b = true;
            }
        }

        assert!(saw_a);
        assert!(saw_b);
    }
}
