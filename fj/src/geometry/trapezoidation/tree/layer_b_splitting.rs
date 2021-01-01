use crate::geometry::trapezoidation::{Edge, Region, Vertex};

use super::layer_a_connections::{
    Node, NodeId, Relation, Tree as InternalTree,
};

/// Tree that knows how to split regions
///
/// This tree adds an additional layer of functionality and assurance: It
/// understands how to split regions, and how to find the right region that an
/// edge or vertex splits. It encapsulates the lower tree layer, leaving the
/// splitting of region nodes as the only way to extend the tree.
pub struct Tree {
    nodes: InternalTree<Branch, Region>,
    root: NodeId,
}

impl Tree {
    pub fn new() -> Self {
        let mut nodes = InternalTree::new();
        let root = nodes.insert_leaf(Region::new());

        Self { nodes, root }
    }

    pub fn root(&self) -> NodeId {
        self.root
    }

    /// Split an existing region
    ///
    /// The provided branch will take its place in the tree. The branch will
    /// have two children, the existing region and a new one.
    pub fn split(&mut self, split_at: NodeId, split_with: Branch) -> NodeId {
        if self.get(&split_at).branch().is_some() {
            panic!("You can only split leafs, not branches");
        }

        // This is the new region.
        let new_leaf_id = self.nodes.insert_leaf(Region::new());

        // We're creating a leaf here, but we'll extend it into a branch in a
        // moment.
        let new_branch_id = self.nodes.insert_leaf(Region::new());

        // Make the new leaf take the place of the one we're about to split,
        // before transforming it into a branch.
        let old_leaf_id = split_at;
        self.nodes.replace_child(&old_leaf_id, &new_branch_id);

        self.nodes.change_leaf_to_branch(
            &new_branch_id,
            split_with,
            &old_leaf_id,
            &new_leaf_id,
        );

        new_branch_id
    }

    pub fn get(&self, id: &NodeId) -> &Node<Branch, Region> {
        self.nodes.get(id)
    }

    pub fn regions(&self) -> impl Iterator<Item = (NodeId, &Region)> + '_ {
        self.nodes.leafs()
    }

    pub fn parent_of(&self, id: &NodeId) -> Option<(NodeId, Relation)> {
        self.nodes.parent_of(id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Branch {
    Edge(Edge),
    Vertex(Vertex),
}

#[cfg(test)]
mod tests {
    use crate::geometry::trapezoidation::{
        tree::layer_a_connections::Relation, Vertex,
    };

    use super::{Branch, Tree};

    #[test]
    fn tree_should_start_with_a_single_root_leaf() {
        let tree = Tree::new();

        let leafs: Vec<_> = tree.regions().collect();
        assert_eq!(leafs.len(), 1);

        let (root_id, _) = leafs[0];
        assert_eq!(root_id, tree.root());
    }

    #[test]
    fn tree_should_split_regions() {
        let mut tree = Tree::new();
        let (original_root_id, _) = tree.regions().next().unwrap();

        let new_node = Branch::Vertex(Vertex::new(0.0, 0.0));
        let new_root_id = tree.split(original_root_id, new_node);

        let leafs: Vec<_> = tree.regions().collect();
        assert_eq!(leafs.len(), 2);

        for (id, _) in leafs {
            let (parent_id, relation) = tree.parent_of(&id).unwrap();

            assert_eq!(parent_id, new_root_id);

            if id == original_root_id {
                assert_eq!(relation, Relation::Above);
            } else {
                assert_eq!(relation, Relation::Below);
            }
        }
    }
}
