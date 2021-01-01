use std::collections::HashMap;

use super::id::{Ids, RawId};

/// Tree that guarantees that the connections between tree nodes are valid
///
/// This true doesn't know anything about the meaning of the nodes it contains.
/// It only knows about the structure of the tree, and makes sure that it is
/// correctly maintained at all times.
pub struct Tree<Branch, Leaf> {
    nodes: HashMap<RawId, Node<Branch, Leaf>>,
    ids: Ids,
}

impl<Branch, Leaf> Tree<Branch, Leaf> {
    pub fn new() -> Self {
        Tree {
            nodes: HashMap::new(),
            ids: Ids::new(),
        }
    }

    pub fn insert_leaf(&mut self, leaf: Leaf) -> NodeId {
        let id = self.ids.next();

        self.nodes
            .insert(id, Node::Leaf(LeafNode { parent: None, leaf }));

        NodeId(id)
    }

    #[cfg(test)]
    pub fn insert_branch(
        &mut self,
        branch: Branch,
        above: &NodeId,
        below: &NodeId,
    ) -> NodeId {
        let id = NodeId(self.ids.next());
        self.insert_branch_internal(&id, branch, None, above, below);
        id
    }

    pub fn replace_child(&mut self, child: &NodeId, new_child: &NodeId) {
        let parent = self.get_mut(child).parent_mut().take();

        *self.get_mut(new_child).parent_mut() = parent;

        if let Some(parent) = parent {
            match self.get_mut(&NodeId(parent)) {
                Node::Branch(branch) if child.0 == branch.above => {
                    branch.above = new_child.0;
                }
                Node::Branch(branch) if child.0 == branch.below => {
                    branch.below = new_child.0;
                }
                Node::Branch(_) => {
                    unreachable!("Parent didn't know about child")
                }
                Node::Leaf(_) => {
                    unreachable!("Parent of a node can't be a leaf")
                }
            }
        }
    }

    pub fn change_leaf_to_branch(
        &mut self,
        id: &NodeId,
        branch: Branch,
        above: &NodeId,
        below: &NodeId,
    ) -> Leaf {
        match self.nodes.remove(&id.0).unwrap() {
            Node::Branch(_) => panic!("Expected leaf, found branch"),
            Node::Leaf(LeafNode { parent, leaf }) => {
                self.insert_branch_internal(&id, branch, parent, above, below);
                leaf
            }
        }
    }

    /// Return a reference to a node
    ///
    /// This can never fail, as nodes are never removed, meaning all node ids
    /// are always valid.
    pub fn get(&self, id: &NodeId) -> &Node<Branch, Leaf> {
        self.nodes.get(&id.0).unwrap()
    }

    /// Return a mutable reference to a node
    ///
    /// This can never fail, as nodes are never removed, meaning all node ids
    /// are always valid.
    pub fn get_mut(&mut self, id: &NodeId) -> &mut Node<Branch, Leaf> {
        self.nodes.get_mut(&id.0).unwrap()
    }

    pub fn parent_of(&self, id: &NodeId) -> Option<(NodeId, Relation)> {
        self.get(id).parent().map(|parent_id| {
            let parent = match self.get(&NodeId(parent_id)) {
                Node::Branch(parent) => parent,
                Node::Leaf(_) => unreachable!("Parent is not a branch"),
            };

            let relation = match id {
                id if id.0 == parent.above => Relation::Above,
                id if id.0 == parent.below => Relation::Below,
                _ => {
                    panic!("Parent doesn't relate to child");
                }
            };
            (NodeId(parent_id), relation)
        })
    }

    pub fn above_of(&self, id: &NodeId) -> NodeId {
        match self.get(id) {
            Node::Branch(BranchNode { above, .. }) => NodeId(*above),
            Node::Leaf(_) => {
                // It would be nicer to enforce this statically, through the use
                // of a branch handle, but for now this will do.
                panic!("Expected branch, got leaf.");
            }
        }
    }

    pub fn below_of(&self, id: &NodeId) -> NodeId {
        match self.get(id) {
            Node::Branch(BranchNode { below, .. }) => NodeId(*below),
            Node::Leaf(_) => {
                // It would be nicer to enforce this statically, through the use
                // of a branch handle, but for now this will do.
                panic!("Expected branch, got leaf.");
            }
        }
    }

    pub fn leafs(&self) -> impl Iterator<Item = (NodeId, &Leaf)> + '_ {
        self.nodes.iter().filter_map(|(&id, node)| match node {
            Node::Leaf(LeafNode { leaf, .. }) => Some((NodeId(id), leaf)),
            _ => None,
        })
    }

    fn insert_branch_internal(
        &mut self,
        id: &NodeId,
        branch: Branch,
        parent: Option<RawId>,
        above: &NodeId,
        below: &NodeId,
    ) {
        // It would be nicer to verify this statically, through the use of some
        // kind of root node handle, but for now this will do.
        assert!(self.get(above).parent().is_none());
        assert!(self.get(below).parent().is_none());

        self.nodes.insert(
            id.0,
            Node::Branch(BranchNode {
                parent,
                above: above.0,
                below: below.0,
                branch,
            }),
        );

        // Update parents of the new children
        *self.get_mut(above).parent_mut() = Some(id.0);
        *self.get_mut(below).parent_mut() = Some(id.0);
    }
}

/// Identifies a node
///
/// Since nodes can only be added, never removed, a `NodeId` instance is always
/// going to be valid.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct NodeId(RawId);

#[derive(Debug, PartialEq)]
pub enum Node<Branch, Leaf> {
    Branch(BranchNode<Branch>),
    Leaf(LeafNode<Leaf>),
}

impl<Branch, Leaf> Node<Branch, Leaf> {
    pub fn branch(&self) -> Option<&Branch> {
        match self {
            Self::Branch(BranchNode { branch, .. }) => Some(branch),
            Self::Leaf(_) => None,
        }
    }

    pub fn branch_mut(&mut self) -> Option<&mut Branch> {
        match self {
            Self::Branch(BranchNode { branch, .. }) => Some(branch),
            Self::Leaf(_) => None,
        }
    }

    pub fn leaf(&self) -> Option<&Leaf> {
        match self {
            Self::Branch(_) => None,
            Self::Leaf(LeafNode { leaf, .. }) => Some(leaf),
        }
    }

    pub fn leaf_mut(&mut self) -> Option<&mut Leaf> {
        match self {
            Self::Branch(_) => None,
            Self::Leaf(LeafNode { leaf, .. }) => Some(leaf),
        }
    }

    fn parent(&self) -> &Option<RawId> {
        match self {
            Self::Branch(BranchNode { parent, .. }) => parent,
            Self::Leaf(LeafNode { parent, .. }) => parent,
        }
    }

    fn parent_mut(&mut self) -> &mut Option<RawId> {
        match self {
            Self::Branch(BranchNode { parent, .. }) => parent,
            Self::Leaf(LeafNode { parent, .. }) => parent,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BranchNode<T> {
    parent: Option<RawId>,
    above: RawId,
    below: RawId,
    branch: T,
}

#[derive(Debug, PartialEq)]
pub struct LeafNode<T> {
    parent: Option<RawId>,
    leaf: T,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Relation {
    Above,
    Below,
}

#[cfg(test)]
mod tests {
    use super::Relation;

    type Tree = super::Tree<u8, u8>;

    #[test]
    fn tree_should_insert_leafs() {
        let mut tree = Tree::new();

        let mut leaf = 5;
        let id = tree.insert_leaf(leaf);

        assert_eq!(tree.get(&id).leaf().unwrap(), &leaf);
        assert_eq!(tree.get_mut(&id).leaf_mut().unwrap(), &mut leaf);

        assert_eq!(tree.parent_of(&id), None);
    }

    #[test]
    fn tree_should_insert_branches() {
        let mut tree = Tree::new();

        let leaf_id_a = tree.insert_leaf(3);
        let leaf_id_b = tree.insert_leaf(5);

        let mut branch = 1;
        let id = tree.insert_branch(branch, &leaf_id_a, &leaf_id_b);

        assert_eq!(tree.get(&id).branch().unwrap(), &branch);
        assert_eq!(tree.get_mut(&id).branch_mut().unwrap(), &mut branch);

        assert_eq!(tree.parent_of(&leaf_id_a), Some((id, Relation::Above)));
        assert_eq!(tree.parent_of(&leaf_id_b), Some((id, Relation::Below)));

        assert_eq!(tree.above_of(&id), leaf_id_a);
        assert_eq!(tree.below_of(&id), leaf_id_b);
    }

    #[test]
    fn tree_should_assign_new_id_when_adding_nodes() {
        let mut tree = Tree::new();

        let id_a = tree.insert_leaf(5);
        let id_b = tree.insert_leaf(8);

        assert_ne!(id_a, id_b);
    }

    #[test]
    fn tree_should_return_all_leafs() {
        let mut tree = Tree::new();

        let leaf_a = 5;
        let leaf_b = 8;

        let id_a = tree.insert_leaf(leaf_a);
        let id_b = tree.insert_leaf(leaf_b);

        let mut saw_a = false;
        let mut saw_b = false;

        for (id, leaf) in tree.leafs() {
            if id == id_a && leaf == &leaf_a {
                saw_a = true;
            }
            if id == id_b && leaf == &leaf_b {
                saw_b = true;
            }
        }

        assert!(saw_a);
        assert!(saw_b);
    }

    #[test]
    fn tree_should_change_root_leaf_to_branch() {
        let mut tree = Tree::new();

        let leaf_tmp = 3;
        let leaf_a = 5;
        let leaf_b = 8;

        let id_branch = tree.insert_leaf(leaf_tmp);
        let id_leaf_a = tree.insert_leaf(leaf_a);
        let id_leaf_b = tree.insert_leaf(leaf_b);

        let mut branch = 1;
        let replaced_leaf = tree
            .change_leaf_to_branch(&id_branch, branch, &id_leaf_a, &id_leaf_b);

        assert_eq!(replaced_leaf, leaf_tmp);

        assert_eq!(tree.get(&id_branch).branch().unwrap(), &branch);
        assert_eq!(tree.get_mut(&id_branch).branch_mut().unwrap(), &mut branch);

        assert_eq!(tree.parent_of(&id_branch), None);
        assert_eq!(
            tree.parent_of(&id_leaf_a),
            Some((id_branch, Relation::Above))
        );
        assert_eq!(
            tree.parent_of(&id_leaf_b),
            Some((id_branch, Relation::Below))
        );
    }

    #[test]
    fn tree_should_change_non_root_leaf_to_branch() {
        let mut tree = Tree::new();

        // Create non-root leaf nodes.
        let root_id = tree.insert_leaf(3);
        let leaf_id_a = tree.insert_leaf(5);
        let leaf_id_b = tree.insert_leaf(8);
        tree.change_leaf_to_branch(&root_id, 1, &leaf_id_a, &leaf_id_b);

        let non_root_leaf_id = leaf_id_a;

        // Change a non-root leaf into a branch
        let leaf_id_a = tree.insert_leaf(13);
        let leaf_id_b = tree.insert_leaf(21);
        tree.change_leaf_to_branch(
            &non_root_leaf_id,
            2,
            &leaf_id_a,
            &leaf_id_b,
        );

        assert_eq!(
            tree.parent_of(&non_root_leaf_id),
            Some((root_id, Relation::Above))
        );
    }

    #[test]
    fn tree_should_replace_children() {
        let mut tree = Tree::new();

        // Create nodes with a parent
        let above_id = tree.insert_leaf(3);
        let below_id = tree.insert_leaf(5);
        let parent_id = tree.insert_branch(1, &above_id, &below_id);

        // Create new nodes that will replace the children
        let above_new_id = tree.insert_leaf(8);
        let below_new_id = tree.insert_leaf(13);

        tree.replace_child(&above_id, &above_new_id);
        assert_eq!(
            tree.parent_of(&above_new_id),
            Some((parent_id, Relation::Above))
        );
        assert_eq!(tree.above_of(&parent_id), above_new_id);

        tree.replace_child(&below_id, &below_new_id);
        assert_eq!(
            tree.parent_of(&below_new_id),
            Some((parent_id, Relation::Below))
        );
        assert_eq!(tree.below_of(&parent_id), below_new_id);

        assert!(tree.parent_of(&above_id).is_none());
        assert!(tree.parent_of(&below_id).is_none());
    }
}
