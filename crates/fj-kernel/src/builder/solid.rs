use fj_math::Scalar;

use crate::{
    objects::{Shell, Solid},
    stores::Stores,
};

/// API for building a [`Solid`]
pub struct SolidBuilder<'a> {
    stores: &'a Stores,
}

impl<'a> SolidBuilder<'a> {
    /// Construct a new instance of `SolidBuilder`
    ///
    /// Also see [`Solid::build`].
    pub fn new(stores: &'a Stores) -> Self {
        Self { stores }
    }

    /// Create a cube from the length of its edges
    pub fn cube_from_edge_length(
        &self,
        edge_length: impl Into<Scalar>,
    ) -> Solid {
        let shell =
            Shell::build(self.stores).cube_from_edge_length(edge_length);
        Solid::new().with_shells([shell])
    }
}
