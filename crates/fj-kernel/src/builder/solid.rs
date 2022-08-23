use fj_math::Scalar;

use crate::objects::{Shell, Solid};

/// API for building a [`Solid`]
pub struct SolidBuilder;

impl SolidBuilder {
    /// Create a cube from the length of its edges
    pub fn cube_from_edge_length(
        &self,
        edge_length: impl Into<Scalar>,
    ) -> Solid {
        let shell = Shell::build().cube_from_edge_length(edge_length);
        Solid::new().with_shells([shell])
    }
}
