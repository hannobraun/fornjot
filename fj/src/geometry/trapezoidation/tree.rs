use super::{
    nodes::{NodeId, Node, Nodes, Relation},
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
        split_at: NodeId,
        split_with: Branch,
    ) -> NodeId {
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

    pub fn get(&self, id: &NodeId) -> &Node<Branch, Trapezoid> {
        self.nodes.get(id)
    }

    pub fn trapezoids(
        &self,
    ) -> impl Iterator<Item = (NodeId, &Trapezoid)> + '_ {
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
        let (original_root_id, _) = tree.trapezoids().next().unwrap();

        let new_node = Branch::Vertex(Vertex::new(0.0, 0.0));
        let new_root_id = tree.split(original_root_id, new_node);

        let leafs: Vec<_> = tree.trapezoids().collect();
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
