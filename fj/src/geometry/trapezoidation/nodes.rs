use std::collections::HashMap;

pub struct Nodes<Branch, Leaf> {
    map: HashMap<RawId, Node<Branch, Leaf>>,
    next_id: RawId,
}

impl<Branch, Leaf> Nodes<Branch, Leaf> {
    pub fn new() -> Self {
        Nodes {
            map: HashMap::new(),
            next_id: 0,
        }
    }

    pub fn insert_leaf(&mut self, leaf: Leaf) -> GenericId {
        let id = self.next_id;
        self.next_id += 1;

        self.map
            .insert(id, Node::Leaf(LeafNode { parent: None, leaf }));

        GenericId(id)
    }

    pub fn change_leaf_to_branch(
        &mut self,
        id: &impl NodeId,
        branch: Branch,
        above: &impl NodeId,
        below: &impl NodeId,
    ) -> Leaf {
        let id = id.raw_id();

        match self.map.remove(&id).unwrap() {
            Node::Branch(_) => panic!("Expected leaf, found branch"),
            Node::Leaf(LeafNode { parent, leaf }) => {
                // It would be nicer to verify this statically, through the use
                // of some kind of root node handle, but for now this will do.
                assert!(self.get(above).parent().is_none());
                assert!(self.get(below).parent().is_none());

                // Insert the new branch node
                self.map.insert(
                    id,
                    Node::Branch(BranchNode {
                        parent,
                        above: above.raw_id(),
                        below: below.raw_id(),
                        branch,
                    }),
                );

                // Update parents of the new children
                *self.get_mut(above).parent_mut() = Some(id);
                *self.get_mut(below).parent_mut() = Some(id);

                leaf
            }
        }
    }

    /// Return a reference to a node
    ///
    /// This can never fail, as nodes are never removed, meaning all node ids
    /// are always valid.
    pub fn get(&self, id: &impl NodeId) -> &Node<Branch, Leaf> {
        self.map.get(&id.raw_id()).unwrap()
    }

    /// Return a mutable reference to a node
    ///
    /// This can never fail, as nodes are never removed, meaning all node ids
    /// are always valid.
    pub fn get_mut(&mut self, id: &impl NodeId) -> &mut Node<Branch, Leaf> {
        self.map.get_mut(&id.raw_id()).unwrap()
    }

    pub fn parent_of(&self, id: &impl NodeId) -> Option<GenericId> {
        self.get(id).parent().map(|id| GenericId(id))
    }

    pub fn leafs(&self) -> impl Iterator<Item = (GenericId, &Leaf)> + '_ {
        self.map.iter().filter_map(|(&id, node)| match node {
            Node::Leaf(LeafNode { leaf, .. }) => Some((GenericId(id), leaf)),
            _ => None,
        })
    }
}

pub trait NodeId {
    fn raw_id(&self) -> RawId;
}

type RawId = u32;

impl NodeId for RawId {
    fn raw_id(&self) -> RawId {
        *self
    }
}

/// Identifies a node
///
/// Since nodes can only be added, never removed, a `NodeId` instance is always
/// going to be valid.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct GenericId(pub RawId);

impl NodeId for GenericId {
    fn raw_id(&self) -> RawId {
        self.0
    }
}

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

    pub fn parent(&self) -> &Option<RawId> {
        match self {
            Node::Branch(BranchNode { parent, .. }) => parent,
            Node::Leaf(LeafNode { parent, .. }) => parent,
        }
    }

    pub fn parent_mut(&mut self) -> &mut Option<RawId> {
        match self {
            Node::Branch(BranchNode { parent, .. }) => parent,
            Node::Leaf(LeafNode { parent, .. }) => parent,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BranchNode<T> {
    pub parent: Option<RawId>,
    pub above: RawId,
    pub below: RawId,
    pub branch: T,
}

#[derive(Debug, PartialEq)]
pub struct LeafNode<T> {
    parent: Option<RawId>,
    pub leaf: T,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Relation {
    Above,
    Below,
}

#[cfg(test)]
mod tests {
    type Nodes = super::Nodes<u8, u8>;

    #[test]
    fn nodes_should_insert_leafs() {
        let mut nodes = Nodes::new();

        let mut leaf = 5;
        let id = nodes.insert_leaf(leaf);

        assert_eq!(nodes.get(&id).leaf().unwrap(), &leaf);
        assert_eq!(nodes.get_mut(&id).leaf_mut().unwrap(), &mut leaf);

        assert_eq!(nodes.parent_of(&id), None);
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
    fn nodes_should_change_root_leaf_to_branch() {
        let mut nodes = Nodes::new();

        let leaf_tmp = 3;
        let leaf_a = 5;
        let leaf_b = 8;

        let id_branch = nodes.insert_leaf(leaf_tmp);
        let id_leaf_a = nodes.insert_leaf(leaf_a);
        let id_leaf_b = nodes.insert_leaf(leaf_b);

        let mut branch = 1;
        let replaced_leaf = nodes
            .change_leaf_to_branch(&id_branch, branch, &id_leaf_a, &id_leaf_b);

        assert_eq!(replaced_leaf, leaf_tmp);

        assert_eq!(nodes.get(&id_branch).branch().unwrap(), &branch);
        assert_eq!(
            nodes.get_mut(&id_branch).branch_mut().unwrap(),
            &mut branch
        );

        assert_eq!(nodes.parent_of(&id_branch), None);
        assert_eq!(nodes.parent_of(&id_leaf_a), Some(id_branch));
        assert_eq!(nodes.parent_of(&id_leaf_b), Some(id_branch));
    }

    #[test]
    fn nodes_should_change_non_root_leaf_to_branch() {
        let mut nodes = Nodes::new();

        // Create non-root leaf nodes.
        let root_id = nodes.insert_leaf(3);
        let leaf_id_a = nodes.insert_leaf(5);
        let leaf_id_b = nodes.insert_leaf(8);
        nodes.change_leaf_to_branch(&root_id, 1, &leaf_id_a, &leaf_id_b);

        let non_root_leaf_id = leaf_id_a;

        // Change a non-root leaf into a branch
        let leaf_id_a = nodes.insert_leaf(13);
        let leaf_id_b = nodes.insert_leaf(21);
        nodes.change_leaf_to_branch(
            &non_root_leaf_id,
            2,
            &leaf_id_a,
            &leaf_id_b,
        );

        assert_eq!(nodes.parent_of(&non_root_leaf_id), Some(root_id));
    }
}
