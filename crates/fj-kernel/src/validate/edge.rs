use fj_math::{Point, Scalar};

use crate::{
    objects::{GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Surface},
    storage::Handle,
};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for HalfEdge {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        HalfEdgeValidationError::check_global_curve_identity(self, errors);
        HalfEdgeValidationError::check_global_vertex_identity(self, errors);
        HalfEdgeValidationError::check_surface_identity(self, errors);
        HalfEdgeValidationError::check_vertex_positions(self, config, errors);

        // We don't need to check anything about surfaces here. We already check
        // curves, which makes sure the vertices are consistent with each other,
        // and the validation of those vertices checks the surfaces.
    }
}

impl Validate for GlobalEdge {
    fn validate_with_config(
        &self,
        _: &ValidationConfig,
        _: &mut Vec<ValidationError>,
    ) {
    }
}

/// [`HalfEdge`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum HalfEdgeValidationError {
    /// [`HalfEdge`]'s [`GlobalCurve`]s do not match
    #[error(
        "Global form of `HalfEdge`'s `Curve` does not match `GlobalCurve` of \n\
        the `HalfEdge`'s `GlobalEdge`\n\
        - `GlobalCurve` from `Curve`: {global_curve_from_curve:#?}\n\
        - `GlobalCurve` from `GlobalEdge`: {global_curve_from_global_form:#?}\n\
        - `HalfEdge`: {half_edge:#?}",
    )]
    GlobalCurveMismatch {
        /// The [`GlobalCurve`] from the [`HalfEdge`]'s `Curve`
        global_curve_from_curve: Handle<GlobalCurve>,

        /// The [`GlobalCurve`] from the [`HalfEdge`]'s global form
        global_curve_from_global_form: Handle<GlobalCurve>,

        /// The half-edge
        half_edge: HalfEdge,
    },

    /// [`HalfEdge`]'s [`GlobalVertex`] objects do not match
    #[error(
        "Global forms of `HalfEdge` vertices do not match vertices of \n\
        `HalfEdge`'s global form\n\
        - `GlobalVertex` from start vertex: {global_vertex_from_half_edge:#?}\n\
        - `GlobalVertex` objects from `GlobalEdge`: \
            {global_vertices_from_global_form:#?}\n\
        - `HalfEdge`: {half_edge:#?}"
    )]
    GlobalVertexMismatch {
        /// The [`GlobalVertex`] from the [`HalfEdge`]'s start vertex
        global_vertex_from_half_edge: Handle<GlobalVertex>,

        /// The [`GlobalCurve`] from the [`HalfEdge`]'s global form
        global_vertices_from_global_form: [Handle<GlobalVertex>; 2],

        /// The half-edge
        half_edge: HalfEdge,
    },

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

    /// [`HalfEdge`]'s vertices are coincident
    #[error(
        "Vertices of `HalfEdge` on curve are coincident\n\
        - Position of back vertex: {back_position:?}\n\
        - Position of front vertex: {front_position:?}\n\
        - `HalfEdge`: {half_edge:#?}"
    )]
    VerticesAreCoincident {
        /// The position of the back vertex
        back_position: Point<1>,

        /// The position of the front vertex
        front_position: Point<1>,

        /// The distance between the two vertices
        distance: Scalar,

        /// The half-edge
        half_edge: HalfEdge,
    },
}

impl HalfEdgeValidationError {
    fn check_global_curve_identity(
        half_edge: &HalfEdge,
        errors: &mut Vec<ValidationError>,
    ) {
        let global_curve_from_curve = half_edge.curve().global_form();
        let global_curve_from_global_form = half_edge.global_form().curve();

        if global_curve_from_curve.id() != global_curve_from_global_form.id() {
            errors.push(
                Box::new(Self::GlobalCurveMismatch {
                    global_curve_from_curve: global_curve_from_curve.clone(),
                    global_curve_from_global_form:
                        global_curve_from_global_form.clone(),
                    half_edge: half_edge.clone(),
                })
                .into(),
            );
        }
    }

    fn check_global_vertex_identity(
        half_edge: &HalfEdge,
        errors: &mut Vec<ValidationError>,
    ) {
        let global_vertex_from_half_edge =
            half_edge.start_vertex().global_form().clone();
        let global_vertices_from_global_form = half_edge
            .global_form()
            .vertices()
            .access_in_normalized_order();

        let matching_global_vertex = global_vertices_from_global_form
            .iter()
            .find(|global_vertex| {
                global_vertex.id() == global_vertex_from_half_edge.id()
            });

        if matching_global_vertex.is_none() {
            errors.push(
                Box::new(Self::GlobalVertexMismatch {
                    global_vertex_from_half_edge,
                    global_vertices_from_global_form,
                    half_edge: half_edge.clone(),
                })
                .into(),
            );
        }
    }

    fn check_surface_identity(
        half_edge: &HalfEdge,
        errors: &mut Vec<ValidationError>,
    ) {
        let curve_surface = half_edge.curve().surface();
        let surface_form_surface = half_edge.start_vertex().surface();

        if curve_surface.id() != surface_form_surface.id() {
            errors.push(
                Box::new(Self::SurfaceMismatch {
                    curve_surface: curve_surface.clone(),
                    surface_form_surface: surface_form_surface.clone(),
                })
                .into(),
            );
        }
    }

    fn check_vertex_positions(
        half_edge: &HalfEdge,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let [back_position, front_position] = half_edge.boundary();
        let distance = (back_position - front_position).magnitude();

        if distance < config.distinct_min_distance {
            errors.push(
                Box::new(Self::VerticesAreCoincident {
                    back_position,
                    front_position,
                    distance,
                    half_edge: half_edge.clone(),
                })
                .into(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use fj_interop::ext::ArrayExt;
    use fj_math::Point;

    use crate::{
        builder::HalfEdgeBuilder,
        insert::Insert,
        objects::{GlobalCurve, HalfEdge},
        partial::{Partial, PartialHalfEdge, PartialObject},
        services::Services,
        validate::Validate,
    };

    #[test]
    fn half_edge_global_curve_mismatch() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let mut half_edge = PartialHalfEdge::default();
            half_edge.update_as_line_segment_from_points(
                services.objects.surfaces.xy_plane(),
                [[0., 0.], [1., 0.]],
            );

            half_edge.build(&mut services.objects)
        };
        let invalid = {
            let global_form = {
                let mut global_edge =
                    Partial::from(valid.global_form().clone());
                global_edge.write().curve =
                    Partial::from(GlobalCurve.insert(&mut services.objects));
                global_edge.build(&mut services.objects)
            };
            let vertices = valid
                .boundary()
                .zip_ext(valid.surface_vertices().map(Clone::clone));

            HalfEdge::new(valid.curve().clone(), vertices, global_form)
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }

    #[test]
    fn half_edge_global_vertex_mismatch() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let mut half_edge = PartialHalfEdge::default();
            half_edge.update_as_line_segment_from_points(
                services.objects.surfaces.xy_plane(),
                [[0., 0.], [1., 0.]],
            );

            half_edge.build(&mut services.objects)
        };
        let invalid = {
            let global_form = {
                let mut global_edge =
                    Partial::from(valid.global_form().clone());
                global_edge.write().vertices = valid
                    .global_form()
                    .vertices()
                    .access_in_normalized_order()
                    // Creating equal but not identical vertices here.
                    .map(|vertex| {
                        Partial::from_partial(
                            Partial::from(vertex).read().clone(),
                        )
                    });
                global_edge.build(&mut services.objects)
            };
            let vertices = valid
                .boundary()
                .zip_ext(valid.surface_vertices().map(Clone::clone));

            HalfEdge::new(valid.curve().clone(), vertices, global_form)
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }

    #[test]
    fn vertex_surface_mismatch() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let mut half_edge = PartialHalfEdge::default();
            half_edge.update_as_line_segment_from_points(
                services.objects.surfaces.xy_plane(),
                [[0., 0.], [1., 0.]],
            );

            half_edge.build(&mut services.objects)
        };
        let invalid = {
            let vertices = valid
                .boundary()
                .zip_ext(valid.surface_vertices())
                .map(|(point, surface_vertex)| {
                    let mut surface_vertex =
                        Partial::from(surface_vertex.clone());
                    surface_vertex.write().surface =
                        Partial::from(services.objects.surfaces.xz_plane());

                    (point, surface_vertex.build(&mut services.objects))
                });

            HalfEdge::new(
                valid.curve().clone(),
                vertices,
                valid.global_form().clone(),
            )
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }

    #[test]
    fn half_edge_vertices_are_coincident() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let mut half_edge = PartialHalfEdge::default();
            half_edge.update_as_line_segment_from_points(
                services.objects.surfaces.xy_plane(),
                [[0., 0.], [1., 0.]],
            );

            half_edge.build(&mut services.objects)
        };
        let invalid = {
            let vertices = valid.surface_vertices().map(|surface_vertex| {
                (Point::from([0.]), surface_vertex.clone())
            });

            HalfEdge::new(
                valid.curve().clone(),
                vertices,
                valid.global_form().clone(),
            )
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }
}
