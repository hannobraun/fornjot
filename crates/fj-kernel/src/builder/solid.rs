use fj_math::Scalar;

use crate::{
    objects::{Shell, Solid},
    stores::Stores,
};

/// API for building a [`Solid`]
///
/// Also see [`Solid::builder`].
pub struct SolidBuilder<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,
}

impl<'a> SolidBuilder<'a> {
    /// Create a cube from the length of its edges
    pub fn build_cube_from_edge_length(
        &self,
        edge_length: impl Into<Scalar>,
    ) -> Solid {
        let shell =
            Shell::builder(self.stores).cube_from_edge_length(edge_length);
        Solid::new().with_shells([shell])
    }
}
