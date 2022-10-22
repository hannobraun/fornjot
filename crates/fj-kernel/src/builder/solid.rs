use fj_math::Scalar;

use crate::objects::{Objects, Shell, Solid};

/// API for building a [`Solid`]
///
/// Also see [`Solid::builder`].
pub struct SolidBuilder<'a> {
    /// The stores that the created objects are put in
    pub objects: &'a Objects,
}

impl<'a> SolidBuilder<'a> {
    /// Create a cube from the length of its edges
    pub fn build_cube_from_edge_length(
        self,
        edge_length: impl Into<Scalar>,
    ) -> Solid {
        let shell = Shell::builder(self.objects)
            .with_cube_from_edge_length(edge_length)
            .build();
        Solid::new().with_shells([shell])
    }
}
