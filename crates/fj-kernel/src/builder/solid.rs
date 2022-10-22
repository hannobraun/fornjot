use std::collections::BTreeSet;

use fj_math::Scalar;

use crate::{
    objects::{Objects, Shell, Solid},
    storage::Handle,
};

/// API for building a [`Solid`]
///
/// Also see [`Solid::builder`].
pub struct SolidBuilder<'a> {
    /// The stores that the created objects are put in
    pub objects: &'a Objects,

    /// The shells that make up the [`Solid`]
    pub shells: BTreeSet<Handle<Shell>>,
}

impl<'a> SolidBuilder<'a> {
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
    ) -> Self {
        let shell = Shell::builder(self.objects)
            .with_cube_from_edge_length(edge_length)
            .build();
        self.shells.insert(shell);
        self
    }

    /// Build the [`Solid`]
    pub fn build(self) -> Solid {
        Solid::new(self.shells)
    }
}
