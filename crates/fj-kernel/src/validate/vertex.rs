use fj_math::{Point, Scalar};

use crate::{
    objects::{GlobalVertex, Surface, SurfaceVertex, Vertex},
    storage::Handle,
};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Vertex {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        VertexValidationError::check_surface_identity(self, errors);
        VertexValidationError::check_position(self, config, errors);
    }
}

impl Validate for SurfaceVertex {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        SurfaceVertexValidationError::check_position(self, config, errors);
    }
}

impl Validate for GlobalVertex {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}

/// [`Vertex`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
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
    fn check_surface_identity(
        vertex: &Vertex,
        errors: &mut Vec<ValidationError>,
    ) {
        let curve_surface = vertex.curve().surface();
        let surface_form_surface = vertex.surface_form().surface();

        if curve_surface.id() != surface_form_surface.id() {
            errors.push(
                Self::SurfaceMismatch {
                    curve_surface: curve_surface.clone(),
                    surface_form_surface: surface_form_surface.clone(),
                }
                .into(),
            );
        }
    }

    fn check_position(
        vertex: &Vertex,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let curve_position_as_surface = vertex
            .curve()
            .path()
            .point_from_path_coords(vertex.position());
        let surface_position = vertex.surface_form().position();

        let distance = curve_position_as_surface.distance_to(&surface_position);

        if distance > config.identical_max_distance {
            errors.push(
                Self::PositionMismatch {
                    vertex: vertex.clone(),
                    surface_vertex: vertex.surface_form().clone_object(),
                    curve_position_as_surface,
                    distance,
                }
                .into(),
            );
        }
    }
}

/// [`SurfaceVertex`] validation error
#[derive(Clone, Debug, thiserror::Error)]
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
        errors: &mut Vec<ValidationError>,
    ) {
        let surface_position_as_global = surface_vertex
            .surface()
            .geometry()
            .point_from_surface_coords(surface_vertex.position());
        let global_position = surface_vertex.global_form().position();

        let distance = surface_position_as_global.distance_to(&global_position);

        if distance > config.identical_max_distance {
            errors.push(
                Self::PositionMismatch {
                    surface_vertex: surface_vertex.clone(),
                    global_vertex: surface_vertex.global_form().clone_object(),
                    surface_position_as_global,
                    distance,
                }
                .into(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builder::CurveBuilder,
        insert::Insert,
        objects::{GlobalVertex, SurfaceVertex, Vertex},
        partial::{
            Partial, PartialCurve, PartialObject, PartialSurfaceVertex,
            PartialVertex,
        },
        services::Services,
        validate::Validate,
    };

    #[test]
    fn vertex_surface_mismatch() -> anyhow::Result<()> {
        let mut services = Services::new();

        let surface = Partial::from(services.objects.surfaces.xy_plane());
        let mut curve = PartialCurve {
            surface: surface.clone(),
            ..Default::default()
        };
        curve.update_as_u_axis();

        let valid = PartialVertex {
            position: Some([0.].into()),
            curve: Partial::from_partial(curve),
            surface_form: Partial::from_partial(PartialSurfaceVertex {
                surface,
                ..Default::default()
            }),
        }
        .build(&mut services.objects);
        let invalid = {
            let mut surface_form = Partial::from(valid.surface_form().clone());
            surface_form.write().surface =
                Partial::from(services.objects.surfaces.xz_plane());
            let surface_form = surface_form.build(&mut services.objects);

            Vertex::new(valid.position(), valid.curve().clone(), surface_form)
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }

    #[test]
    fn vertex_position_mismatch() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let surface = Partial::from(services.objects.surfaces.xy_plane());
            let mut curve = PartialCurve {
                surface: surface.clone(),
                ..Default::default()
            };
            curve.update_as_u_axis();

            PartialVertex {
                position: Some([0.].into()),
                curve: Partial::from_partial(curve),
                surface_form: Partial::from_partial(PartialSurfaceVertex {
                    surface,
                    ..Default::default()
                }),
            }
            .build(&mut services.objects)
        };
        let invalid = {
            let mut surface_form = Partial::from(valid.surface_form().clone());
            surface_form.write().position = Some([1., 0.].into());
            surface_form.write().global_form = Partial::new();
            let surface_form = surface_form.build(&mut services.objects);

            Vertex::new(valid.position(), valid.curve().clone(), surface_form)
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }

    #[test]
    fn surface_vertex_position_mismatch() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = PartialSurfaceVertex {
            position: Some([0., 0.].into()),
            surface: Partial::from(services.objects.surfaces.xy_plane()),
            ..Default::default()
        }
        .build(&mut services.objects);
        let invalid = SurfaceVertex::new(
            valid.position(),
            valid.surface().clone(),
            GlobalVertex::new([1., 0., 0.]).insert(&mut services.objects),
        );

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }
}
