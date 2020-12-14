use std::collections::HashMap;

pub struct Tree<Branch, Leaf> {
    nodes: HashMap<NodeId, Node<Branch, Leaf>>,
    next_id: u32,
}

impl<Branch, Leaf> Tree<Branch, Leaf>
where
    Leaf: Default,
{
    pub fn new() -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(
            NodeId(0),
            Node {
                parent: None,
                kind: NodeKind::Leaf(Leaf::default()),
            },
        );

        Self { nodes, next_id: 1 }
    }

    pub fn split(&mut self, split_at: LeafId, split_with: Branch) -> NodeId {
        // Generate new ids. We need to do this before we can add the new nodes,
        // as those nodes reference each other.
        let new_branch_id = self.next_id();
        let new_leaf_id = self.next_id();

        // Update the old leaf we're splitting.
        let old_leaf_id = split_at;
        let old_leaf = self.get_mut(old_leaf_id.0);
        let old_leaf_parent = old_leaf.parent;
        old_leaf.parent = Some(new_branch_id);

        // Update the old leaf's parent, if it has one.
        if let Some(parent_id) = old_leaf_parent {
            let parent = self.get_parent_mut(parent_id);
            match old_leaf_id {
                id if id.0 == parent.above => parent.above = new_branch_id,
                id if id.0 == parent.below => parent.above = new_branch_id,
                id => panic!(
                    "Parent ({:?}) of split leaf ({:?}) doesn't relate to it",
                    old_leaf_parent, id
                ),
            }
        }

        // Insert the new nodes.
        self.nodes.insert(
            new_branch_id,
            Node {
                parent: old_leaf_parent,
                kind: NodeKind::Branch(BranchNode {
                    above: old_leaf_id.0,
                    below: new_leaf_id,
                    branch: split_with,
                }),
            },
        );
        self.nodes.insert(
            new_leaf_id,
            Node {
                parent: Some(new_branch_id),
                kind: NodeKind::Leaf(Leaf::default()),
            },
        );

        new_branch_id
    }

    pub fn leafs(&self) -> impl Iterator<Item = (LeafId, &Leaf)> + '_ {
        self.nodes
            .iter()
            .filter_map(|(&id, node)| match &node.kind {
                NodeKind::Leaf(leaf) => Some((LeafId(id), leaf)),
                _ => None,
            })
    }

    pub fn parent_of(
        &self,
        id: impl Into<NodeId>,
    ) -> Option<(NodeId, &Branch, Relation)> {
        let id = id.into();

        let node = self.get(id);
        node.parent.map(|parent_id| {
            let parent = self.get_parent(parent_id);

            let relation = match id {
                id if id == parent.above => Relation::Above,
                id if id == parent.below => Relation::Below,
                id => {
                    panic!(
                        "Parent ({:?}) doesn't relate to child {:?}",
                        parent_id, id
                    );
                }
            };

            (parent_id, &parent.branch, relation)
        })
    }

    fn get(&self, id: NodeId) -> &Node<Branch, Leaf> {
        self.nodes.get(&id).unwrap()
    }

    fn get_mut(&mut self, id: NodeId) -> &mut Node<Branch, Leaf> {
        self.nodes.get_mut(&id).unwrap()
    }

    fn get_parent(&self, parent_id: NodeId) -> &BranchNode<Branch> {
        if let NodeKind::Branch(node) = &self.get(parent_id).kind {
            return node;
        }

        panic!("Parent node ({:?}) is not a branch", parent_id);
    }

    fn get_parent_mut(&mut self, parent_id: NodeId) -> &mut BranchNode<Branch> {
        if let NodeKind::Branch(node) = &mut self.get_mut(parent_id).kind {
            return node;
        }

        panic!("Parent node ({:?}) is not a branch", parent_id);
    }

    fn next_id(&mut self) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;
        id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct NodeId(u32);

impl From<LeafId> for NodeId {
    fn from(leaf_id: LeafId) -> Self {
        leaf_id.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct LeafId(NodeId);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Node<Branch, Leaf> {
    parent: Option<NodeId>,
    kind: NodeKind<Branch, Leaf>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeKind<Branch, Leaf> {
    Branch(BranchNode<Branch>),
    Leaf(Leaf),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BranchNode<T> {
    above: NodeId,
    below: NodeId,
    branch: T,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Relation {
    Above,
    Below,
}

#[cfg(test)]
mod tests {
    use super::Relation;

    type Tree = super::Tree<u8, ()>;

    #[test]
    fn tree_should_start_with_a_single_root_trapezoid() {
        let tree = Tree::new();

        let num_trapezoids = tree.leafs().count();
        assert_eq!(num_trapezoids, 1);
    }

    #[test]
    fn tree_should_split_trapezoids() {
        let mut tree = Tree::new();
        let (root_id, _) = tree.leafs().next().unwrap();

        let new_node = 0;
        tree.split(root_id, new_node);

        let trapezoids: Vec<_> = tree.leafs().collect();
        assert_eq!(trapezoids.len(), 2);

        // This is no longer the root, so let's update the variable name.
        let original_root_id = root_id;

        for (id, _) in trapezoids {
            let (_, parent, relation) = tree.parent_of(id).unwrap();

            assert_eq!(parent, &new_node);

            if id == original_root_id {
                assert_eq!(relation, Relation::Above);
            } else {
                assert_eq!(relation, Relation::Below);
            }
        }

        // Make sure that the new branch node has the same parent as the
        // previous leaf node.

        let (leaf_id, _) = tree.leafs().next().unwrap();
        let (parent_id, _, _) = tree.parent_of(leaf_id).unwrap();
        let new_branch_id = tree.split(leaf_id, 1);
        let (new_parent_id, _, _) = tree.parent_of(new_branch_id).unwrap();

        assert_eq!(parent_id, new_parent_id);
    }
}
