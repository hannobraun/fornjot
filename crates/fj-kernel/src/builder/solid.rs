use fj_math::Scalar;

use crate::{
    objects::{Objects, Shell},
    partial::{Partial, PartialShell, PartialSolid},
    services::Service,
};

use super::ShellBuilder;

/// Builder API for [`PartialSolid`]
pub trait SolidBuilder {
    /// Build the [`Solid`] with the provided shells
    fn with_shells(
        self,
        shells: impl IntoIterator<Item = impl Into<Partial<Shell>>>,
    ) -> Self;

    /// Create a cube from the length of its edges
    fn with_cube_from_edge_length(
        self,
        edge_length: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    ) -> Self;
}

impl SolidBuilder for PartialSolid {
    fn with_shells(
        mut self,
        shells: impl IntoIterator<Item = impl Into<Partial<Shell>>>,
    ) -> Self {
        let shells = shells.into_iter().map(Into::into);
        self.shells.extend(shells);
        self
    }

    fn with_cube_from_edge_length(
        mut self,
        edge_length: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    ) -> Self {
        let shell =
            PartialShell::create_cube_from_edge_length(edge_length, objects);
        self.shells.push(Partial::from_partial(shell));
        self
    }
}
