use std::collections::BTreeSet;

use fj_math::Scalar;

use crate::{
    insert::Insert,
    objects::{Objects, Shell, Solid},
    services::Service,
    storage::Handle,
};

/// API for building a [`Solid`]
///
/// Also see [`Solid::builder`].
pub struct SolidBuilder {
    /// The shells that make up the [`Solid`]
    pub shells: BTreeSet<Handle<Shell>>,
}

impl SolidBuilder {
    /// Build the [`Solid`] with the provided shells
    pub fn with_shells(
        mut self,
        shells: impl IntoIterator<Item = Handle<Shell>>,
    ) -> Self {
        self.shells.extend(shells);
        self
    }

    /// Create a cube from the length of its edges
    pub fn with_cube_from_edge_length(
        mut self,
        edge_length: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    ) -> Self {
        let shell = Shell::builder()
            .with_cube_from_edge_length(edge_length, objects)
            .build(objects);
        self.shells.insert(shell);
        self
    }

    /// Build the [`Solid`]
    pub fn build(self, objects: &mut Service<Objects>) -> Handle<Solid> {
        Solid::new(self.shells).insert(objects).unwrap()
    }
}
