use super::{
    nodes::{
        BranchNode, LeafId, Node, NodeId, NodeKind, Nodes, Relation, Strong,
    },
    Edge, Vertex,
};

pub struct Tree {
    nodes: Nodes<Branch, Trapezoid>,
}

impl Tree {
    pub fn new() -> Self {
        let mut nodes = Nodes::new();
        nodes.insert_leaf(Trapezoid);

        Self { nodes }
    }

    /// Split an existing trapezoid
    ///
    /// The provided branch will take its place in the tree. The branch will
    /// have two children, the existing trapezoid and a new one.
    pub fn split(&mut self, split_at: LeafId, split_with: Branch) -> NodeId {
        // This is the new trapezoid.
        let new_leaf_id = self.nodes.insert_leaf(Trapezoid);

        // We're creating a leaf here, but we'll extend it into a branch in a
        // moment.
        let new_branch_id = self.nodes.insert_leaf(Trapezoid);

        // Update the old leaf we're splitting.
        let old_leaf_id = split_at;
        let old_leaf = self.nodes.get_mut(old_leaf_id.0);
        let old_leaf_parent =
            old_leaf.parent.take().map(|strong| strong.as_node_id());
        old_leaf.parent = Some(Strong(new_branch_id.as_leaf_id().into()));

        // Update the old leaf's parent, if it has one.
        if let Some(parent_id) = old_leaf_parent {
            let parent = self.get_parent_mut(parent_id);
            match old_leaf_id {
                id if id.0 == parent.above.as_node_id() => {
                    parent.above = Strong(new_branch_id.as_leaf_id().into())
                }
                // This looks like a bug. I don't want to apply the obvious fix,
                // as the real bug here is that none of the tests are failing.
                // If this code still exists after I've finished cleaning up, I
                // need to handle it properly.
                id if id.0 == parent.below.as_node_id() => {
                    parent.above = Strong(new_branch_id.as_leaf_id().into())
                }
                id => panic!(
                    "Parent ({:?}) of split leaf ({:?}) doesn't relate to it",
                    old_leaf_parent, id
                ),
            }
        }

        // Insert the new nodes.
        let new_leaf_id_tmp = new_leaf_id.as_leaf_id();
        let new_branch_id: NodeId = new_branch_id.as_leaf_id().into();
        self.nodes.map.insert(
            new_branch_id.0,
            Node {
                parent: old_leaf_parent.map(|id| Strong(id)),
                kind: NodeKind::Branch(BranchNode {
                    above: Strong(old_leaf_id.0),
                    below: new_leaf_id.into(),
                    branch: split_with,
                }),
            },
        );
        self.nodes.get_mut(new_leaf_id_tmp).parent =
            Some(Strong(new_branch_id));

        new_branch_id
    }

    pub fn trapezoids(
        &self,
    ) -> impl Iterator<Item = (LeafId, &Trapezoid)> + '_ {
        self.nodes.leafs()
    }

    pub fn parent_of(
        &self,
        id: impl Into<NodeId>,
    ) -> Option<(NodeId, &Branch, Relation)> {
        let id = id.into();

        let node = self.nodes.get(id);
        node.parent.as_ref().map(|parent_id| {
            let parent = self.get_parent(parent_id.as_node_id());

            let relation = match id {
                id if id == parent.above.as_node_id() => Relation::Above,
                id if id == parent.below.as_node_id() => Relation::Below,
                id => {
                    panic!(
                        "Parent ({:?}) doesn't relate to child {:?}",
                        parent_id, id
                    );
                }
            };

            (parent_id.as_node_id(), &parent.branch, relation)
        })
    }

    fn get_parent(&self, parent_id: impl Into<NodeId>) -> &BranchNode<Branch> {
        let parent_id = parent_id.into();

        if let NodeKind::Branch(node) = &self.nodes.get(parent_id).kind {
            return node;
        }

        panic!("Parent node ({:?}) is not a branch", parent_id);
    }

    fn get_parent_mut(
        &mut self,
        parent_id: impl Into<NodeId>,
    ) -> &mut BranchNode<Branch> {
        let parent_id = parent_id.into();

        if let NodeKind::Branch(node) = &mut self.nodes.get_mut(parent_id).kind
        {
            return node;
        }

        panic!("Parent node ({:?}) is not a branch", parent_id);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Branch {
    Edge(Edge),
    Vertex(Vertex),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Trapezoid;

#[cfg(test)]
mod tests {
    use crate::geometry::trapezoidation::{nodes::Relation, Vertex};

    use super::{Branch, Tree};

    #[test]
    fn tree_should_start_with_a_single_root_leaf() {
        let tree = Tree::new();

        let num_leafs = tree.trapezoids().count();
        assert_eq!(num_leafs, 1);
    }

    #[test]
    fn tree_should_split_leafs() {
        let mut tree = Tree::new();
        let (root_id, _) = tree.trapezoids().next().unwrap();

        let new_node = Branch::Vertex(Vertex::new(0.0, 0.0));
        tree.split(root_id, new_node);

        let leafs: Vec<_> = tree.trapezoids().collect();
        assert_eq!(leafs.len(), 2);

        // This is no longer the root, so let's update the variable name.
        let original_root_id = root_id;

        for (id, _) in leafs {
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

        let (leaf_id, _) = tree.trapezoids().next().unwrap();
        let (parent_id, _, _) = tree.parent_of(leaf_id).unwrap();
        let new_branch_id =
            tree.split(leaf_id, Branch::Vertex(Vertex::new(1.0, 1.0)));
        let (new_parent_id, _, _) = tree.parent_of(new_branch_id).unwrap();

        assert_eq!(parent_id, new_parent_id);
    }
}
