//! The trapezoidation tree
//!
//! During the trapezoidation process, the edges and vertices of the polygon are
//! inserted into the tree. The tree updates the metadata associated with each
//! trapezoid, allowing for later phases to process the trapezoids further.

mod id;

// The trapezoidation tree is implemented in multiple layers. Each layer knows
// about a specific aspect of the tree's structure and functionality and
// provides the layer above a solid base to build upon.
mod layer_a_connections;

use layer_a_connections::{Node, NodeId, Relation};

use super::{Edge, Trapezoid, Vertex};

pub struct Tree {
    nodes: layer_a_connections::Tree<Branch, Trapezoid>,
    root: NodeId,
}

impl Tree {
    pub fn new() -> Self {
        let mut nodes = layer_a_connections::Tree::new();
        let root = nodes.insert_leaf(Trapezoid::new());

        Self { nodes, root }
    }

    pub fn root(&self) -> NodeId {
        self.root
    }

    /// Split an existing trapezoid
    ///
    /// The provided branch will take its place in the tree. The branch will
    /// have two children, the existing trapezoid and a new one.
    pub fn split(&mut self, split_at: NodeId, split_with: Branch) -> NodeId {
        if self.get(&split_at).branch().is_some() {
            panic!("You can only split trapezoids, not branches");
        }

        // This is the new trapezoid.
        let new_leaf_id = self.nodes.insert_leaf(Trapezoid::new());

        // We're creating a leaf here, but we'll extend it into a branch in a
        // moment.
        let new_branch_id = self.nodes.insert_leaf(Trapezoid::new());

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

#[cfg(test)]
mod tests {
    use crate::geometry::trapezoidation::Vertex;

    use super::{layer_a_connections::Relation, Branch, Tree};

    #[test]
    fn tree_should_start_with_a_single_root_leaf() {
        let tree = Tree::new();

        let leafs: Vec<_> = tree.trapezoids().collect();
        assert_eq!(leafs.len(), 1);

        let (root_id, _) = leafs[0];
        assert_eq!(root_id, tree.root());
    }

    #[test]
    fn tree_should_split_trapezoids() {
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
