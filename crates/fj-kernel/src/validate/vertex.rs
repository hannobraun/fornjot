use std::{convert::Infallible, ops::Deref};

use fj_math::{Point, Scalar};

use crate::{
    objects::{GlobalVertex, Surface, SurfaceVertex, Vertex},
    storage::Handle,
};

use super::{Validate2, ValidationConfig};

impl Validate2 for Vertex {
    type Error = VertexValidationError;

    fn validate_with_config(
        &self,
        config: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        VertexValidationError::check_surface_identity(self)?;

        let curve_position_as_surface =
            self.curve().path().point_from_path_coords(self.position());
        let surface_position = self.surface_form().position();

        let distance = curve_position_as_surface.distance_to(&surface_position);

        if distance > config.identical_max_distance {
            return Err(VertexValidationError::PositionMismatch {
                vertex: self.clone(),
                surface_vertex: self.surface_form().clone_object(),
                curve_position_as_surface,
                distance,
            });
        }

        Ok(())
    }
}

/// [`Vertex`] validation failed
#[derive(Debug, thiserror::Error)]
pub enum VertexValidationError {
    /// Mismatch between the surface's of the curve and surface form
    #[error(
        "Surface form of vertex must be defined on same surface as curve\n\
        `Surface` of curve: {curve_surface:?} -> {:?}\n\
        `Surface` of surface form: {surface_form_surface:?} -> {:?}",
        .curve_surface.deref(),
        .surface_form_surface.deref(),
    )]
    SurfaceMismatch {
        /// The surface of the vertex' curve
        curve_surface: Handle<Surface>,

        /// The surface of the vertex' surface form
        surface_form_surface: Handle<Surface>,
    },

    /// Mismatch between position of the vertex and position of its surface form
    #[error(
        "`Vertex` position doesn't match position of its surface form\n\
        `Vertex`: {vertex:#?}\n\
        `SurfaceVertex`: {surface_vertex:#?}\n\
        `Vertex` position as surface: {curve_position_as_surface:?}\n\
        Distance between the positions: {distance}"
    )]
    PositionMismatch {
        /// The vertex
        vertex: Vertex,

        /// The mismatched surface vertex
        surface_vertex: SurfaceVertex,

        /// The curve position converted into a surface position
        curve_position_as_surface: Point<2>,

        /// The distance between the positions
        distance: Scalar,
    },
}

impl VertexValidationError {
    fn check_surface_identity(vertex: &Vertex) -> Result<(), Self> {
        let curve_surface = vertex.curve().surface();
        let surface_form_surface = vertex.surface_form().surface();

        if curve_surface.id() != surface_form_surface.id() {
            return Err(VertexValidationError::SurfaceMismatch {
                curve_surface: curve_surface.clone(),
                surface_form_surface: surface_form_surface.clone(),
            });
        }

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

/// Mismatch between position of surface vertex and position of its global form
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
        objects::{Curve, GlobalVertex, Objects, SurfaceVertex, Vertex},
        partial::HasPartial,
        validate::Validate2,
    };

    #[test]
    fn vertex_surface_mismatch() -> anyhow::Result<()> {
        let objects = Objects::new();

        let valid = Vertex::new(
            [0.],
            Curve::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .as_u_axis()
                .build(&objects)?,
            SurfaceVertex::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .with_position(Some([0., 0.]))
                .build(&objects)?,
        );
        let invalid = Vertex::new(
            [0.],
            Curve::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .as_u_axis()
                .build(&objects)?,
            SurfaceVertex::partial()
                .with_surface(Some(objects.surfaces.xz_plane()))
                .with_position(Some([0., 0.]))
                .build(&objects)?,
        );

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());

        Ok(())
    }

    #[test]
    fn vertex_position_mismatch() -> anyhow::Result<()> {
        let objects = Objects::new();

        let valid = Vertex::new(
            [0.],
            Curve::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .as_u_axis()
                .build(&objects)?,
            SurfaceVertex::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .with_position(Some([0., 0.]))
                .build(&objects)?,
        );
        let invalid = Vertex::new(
            [0.],
            Curve::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .as_u_axis()
                .build(&objects)?,
            SurfaceVertex::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .with_position(Some([1., 0.]))
                .build(&objects)?,
        );

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());

        Ok(())
    }

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
