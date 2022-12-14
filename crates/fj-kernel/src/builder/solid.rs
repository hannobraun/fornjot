use fj_math::Scalar;

use crate::{
    objects::Objects,
    partial::{Partial, PartialShell, PartialSolid},
    services::Service,
};

use super::ShellBuilder;

/// Builder API for [`PartialSolid`]
pub trait SolidBuilder {
    /// Add a cube with the given edge length to the solid
    fn with_cube_from_edge_length(
        &mut self,
        edge_length: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    );
}

impl SolidBuilder for PartialSolid {
    fn with_cube_from_edge_length(
        &mut self,
        edge_length: impl Into<Scalar>,
        objects: &mut Service<Objects>,
    ) {
        let shell =
            PartialShell::create_cube_from_edge_length(edge_length, objects);
        self.shells.push(Partial::from_partial(shell));
    }
}
