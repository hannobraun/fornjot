#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct NodeId(pub u32);

impl From<LeafId> for NodeId {
    fn from(leaf_id: LeafId) -> Self {
        leaf_id.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LeafId(pub NodeId);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Node<Branch, Leaf> {
    pub parent: Option<NodeId>,
    pub kind: NodeKind<Branch, Leaf>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeKind<Branch, Leaf> {
    Branch(BranchNode<Branch>),
    Leaf(Leaf),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BranchNode<T> {
    pub above: NodeId,
    pub below: NodeId,
    pub branch: T,
}
