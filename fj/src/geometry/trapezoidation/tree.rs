use super::{
    id::NodeId,
    nodes::{GenericId, Node, Nodes, Relation},
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
    pub fn split(
        &mut self,
        split_at: GenericId,
        split_with: Branch,
    ) -> GenericId {
        // This is the new trapezoid.
        let new_leaf_id = self.nodes.insert_leaf(Trapezoid);

        // We're creating a leaf here, but we'll extend it into a branch in a
        // moment.
        let new_branch_id = self.nodes.insert_leaf(Trapezoid);

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

    pub fn get(&self, id: &impl NodeId) -> &Node<Branch, Trapezoid> {
        self.nodes.get(id)
    }

    pub fn trapezoids(
        &self,
    ) -> impl Iterator<Item = (GenericId, &Trapezoid)> + '_ {
        self.nodes.leafs()
    }

    pub fn parent_of(&self, id: &impl NodeId) -> Option<(GenericId, Relation)> {
        self.nodes.parent_of(id)
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
            let (parent_id, relation) = tree.parent_of(&id).unwrap();
            let parent = tree.get(&parent_id);

            assert_eq!(parent.branch().unwrap(), &new_node);

            if id == original_root_id {
                assert_eq!(relation, Relation::Above);
            } else {
                assert_eq!(relation, Relation::Below);
            }
        }

        // Make sure that the new branch node has the same parent as the
        // previous leaf node.

        let (leaf_id, _) = tree.trapezoids().next().unwrap();
        let (parent_id, _) = tree.parent_of(&leaf_id).unwrap();
        let new_branch_id =
            tree.split(leaf_id, Branch::Vertex(Vertex::new(1.0, 1.0)));
        let (new_parent_id, _) = tree.parent_of(&new_branch_id).unwrap();

        assert_eq!(parent_id, new_parent_id);
    }
}
