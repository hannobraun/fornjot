use std::convert::Infallible;

use fj_math::{Point, Scalar};

use crate::objects::{GlobalVertex, SurfaceVertex, Vertex};

use super::{Validate2, ValidationConfig};

impl Validate2 for Vertex {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Validate2 for SurfaceVertex {
    type Error = SurfaceVertexPositionMismatch;

    fn validate_with_config(
        &self,
        config: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        let surface_position_as_global =
            self.surface().point_from_surface_coords(self.position());
        let global_position = self.global_form().position();

        let distance = surface_position_as_global.distance_to(&global_position);

        if distance > config.identical_max_distance {
            return Err(SurfaceVertexPositionMismatch {
                surface_vertex: self.clone(),
                global_vertex: self.global_form().clone_object(),
                surface_position_as_global,
                distance,
            });
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
#[error(
    "`SurfaceVertex` position doesn't match position of its global form\n\
    `SurfaceVertex`: {surface_vertex:#?}\n\
    `GlobalVertex`: {global_vertex:#?}\n\
    `SurfaceVertex` position as global: {surface_position_as_global:?}\n\
    Distance between the positions: {distance}"
)]
pub struct SurfaceVertexPositionMismatch {
    /// The surface vertex
    pub surface_vertex: SurfaceVertex,

    /// The mismatched global vertex
    pub global_vertex: GlobalVertex,

    /// The surface position converted into a global position
    pub surface_position_as_global: Point<3>,

    /// The distance between the positions
    pub distance: Scalar,
}

impl Validate2 for GlobalVertex {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        objects::{GlobalVertex, Objects, SurfaceVertex},
        validate::Validate2,
    };

    #[test]
    fn surface_vertex_position_mismatch() -> anyhow::Result<()> {
        let objects = Objects::new();

        let valid = SurfaceVertex::new(
            [0., 0.],
            objects.surfaces.xy_plane(),
            objects
                .global_vertices
                .insert(GlobalVertex::from_position([0., 0., 0.]))?,
        );
        let invalid = SurfaceVertex::new(
            [0., 0.],
            objects.surfaces.xy_plane(),
            objects
                .global_vertices
                .insert(GlobalVertex::from_position([1., 0., 0.]))?,
        );

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());

        Ok(())
    }
}
