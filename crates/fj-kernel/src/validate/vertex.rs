use std::convert::Infallible;

use fj_math::{Point, Scalar};

use crate::{
    objects::{GlobalVertex, Surface, SurfaceVertex, Vertex},
    storage::Handle,
};

use super::{Validate, ValidationConfig};

impl Validate for Vertex {
    type Error = VertexValidationError;

    fn validate_with_config(
        &self,
        config: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        VertexValidationError::check_surface_identity(self)?;
        VertexValidationError::check_position(self, config)?;
        Ok(())
    }
}

impl Validate for SurfaceVertex {
    type Error = SurfaceVertexValidationError;

    fn validate_with_config(
        &self,
        config: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        SurfaceVertexValidationError::check_position(self, config)?;
        Ok(())
    }
}

impl Validate for GlobalVertex {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// [`Vertex`] validation failed
#[derive(Debug, thiserror::Error)]
pub enum VertexValidationError {
    /// Mismatch between the surface's of the curve and surface form
    #[error(
        "Surface form of vertex must be defined on same surface as curve\n\
        - `Surface` of curve: {curve_surface:#?}\n\
        - `Surface` of surface form: {surface_form_surface:#?}"
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
        - `Vertex`: {vertex:#?}\n\
        - `SurfaceVertex`: {surface_vertex:#?}\n\
        - `Vertex` position as surface: {curve_position_as_surface:?}\n\
        - Distance between the positions: {distance}"
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

    fn check_position(
        vertex: &Vertex,
        config: &ValidationConfig,
    ) -> Result<(), Self> {
        let curve_position_as_surface = vertex
            .curve()
            .path()
            .point_from_path_coords(vertex.position());
        let surface_position = vertex.surface_form().position();

        let distance = curve_position_as_surface.distance_to(&surface_position);

        if distance > config.identical_max_distance {
            return Err(VertexValidationError::PositionMismatch {
                vertex: vertex.clone(),
                surface_vertex: vertex.surface_form().clone_object(),
                curve_position_as_surface,
                distance,
            });
        }

        Ok(())
    }
}

/// [`SurfaceVertex`] validation error
#[derive(Debug, thiserror::Error)]
pub enum SurfaceVertexValidationError {
    /// Mismatch between position and position of global form
    #[error(
        "`SurfaceVertex` position doesn't match position of its global form\n\
    - `SurfaceVertex`: {surface_vertex:#?}\n\
    - `GlobalVertex`: {global_vertex:#?}\n\
    - `SurfaceVertex` position as global: {surface_position_as_global:?}\n\
    - Distance between the positions: {distance}"
    )]
    PositionMismatch {
        /// The surface vertex
        surface_vertex: SurfaceVertex,

        /// The mismatched global vertex
        global_vertex: GlobalVertex,

        /// The surface position converted into a global position
        surface_position_as_global: Point<3>,

        /// The distance between the positions
        distance: Scalar,
    },
}

impl SurfaceVertexValidationError {
    fn check_position(
        surface_vertex: &SurfaceVertex,
        config: &ValidationConfig,
    ) -> Result<(), Self> {
        let surface_position_as_global = surface_vertex
            .surface()
            .point_from_surface_coords(surface_vertex.position());
        let global_position = surface_vertex.global_form().position();

        let distance = surface_position_as_global.distance_to(&global_position);

        if distance > config.identical_max_distance {
            return Err(Self::PositionMismatch {
                surface_vertex: surface_vertex.clone(),
                global_vertex: surface_vertex.global_form().clone_object(),
                surface_position_as_global,
                distance,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builder::{CurveBuilder, SurfaceVertexBuilder},
        insert::Insert,
        objects::{GlobalVertex, Objects, SurfaceVertex, Vertex},
        partial::{HasPartial, PartialCurve, PartialSurfaceVertex},
        validate::Validate,
    };

    #[test]
    fn vertex_surface_mismatch() -> anyhow::Result<()> {
        let objects = Objects::new();

        let mut curve = PartialCurve {
            surface: Some(objects.surfaces.xy_plane()),
            ..Default::default()
        };
        curve.update_as_u_axis();

        let valid = Vertex::partial()
            .with_position(Some([0.]))
            .with_curve(curve)
            .build(&objects)?;
        let invalid = Vertex::new(valid.position(), valid.curve().clone(), {
            let mut tmp = valid.surface_form().to_partial();
            tmp.surface = Some(objects.surfaces.xz_plane());
            tmp.build(&objects)?.insert(&objects)?
        });

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());

        Ok(())
    }

    #[test]
    fn vertex_position_mismatch() -> anyhow::Result<()> {
        let objects = Objects::new();

        let valid = {
            let mut curve = PartialCurve {
                surface: Some(objects.surfaces.xy_plane()),
                ..Default::default()
            };
            curve.update_as_u_axis();

            Vertex::partial()
                .with_position(Some([0.]))
                .with_curve(curve)
                .build(&objects)?
        };
        let invalid = Vertex::new(valid.position(), valid.curve().clone(), {
            let mut tmp = valid.surface_form().to_partial();
            tmp.position = Some([1., 0.].into());
            tmp.infer_global_form();
            tmp.build(&objects)?.insert(&objects)?
        });

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());

        Ok(())
    }

    #[test]
    fn surface_vertex_position_mismatch() -> anyhow::Result<()> {
        let objects = Objects::new();

        let valid = PartialSurfaceVertex {
            position: Some([0., 0.].into()),
            surface: Some(objects.surfaces.xy_plane()),
            ..Default::default()
        }
        .build(&objects)?;
        let invalid = SurfaceVertex::new(
            valid.position(),
            valid.surface().clone(),
            objects
                .global_vertices
                .insert(GlobalVertex::from_position([1., 0., 0.]))?,
        );

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());

        Ok(())
    }
}
