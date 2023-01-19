use fj_interop::ext::ArrayExt;
use fj_math::{Point, Scalar};

use crate::{
    objects::{
        GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Surface, Vertex,
        VerticesInNormalizedOrder,
    },
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
        HalfEdgeValidationError::check_position(self, config, errors);

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
        - `GlobalVertex` objects from `Vertex` objects: \
            {global_vertices_from_vertices:#?}\n\
        - `GlobalVertex` objects from `GlobalEdge`: \
            {global_vertices_from_global_form:#?}\n\
        - `HalfEdge`: {half_edge:#?}"
    )]
    GlobalVertexMismatch {
        /// The [`GlobalVertex`] from the [`HalfEdge`]'s vertices
        global_vertices_from_vertices: [Handle<GlobalVertex>; 2],

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

    /// Mismatch between position of the vertex and position of its surface form
    #[error(
        "`Vertex` position doesn't match position of its surface form\n\
        - `Vertex`: {vertex:#?}\n\
        - `Vertex` position on surface: {curve_position_on_surface:?}\n\
        - Distance between the positions: {distance}"
    )]
    VertexPositionMismatch {
        /// The vertex
        vertex: Vertex,

        /// The curve position converted into a surface position
        curve_position_on_surface: Point<2>,

        /// The distance between the positions
        distance: Scalar,
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
        let global_vertices_from_vertices = {
            let (global_vertices_from_vertices, _) =
                VerticesInNormalizedOrder::new(
                    half_edge
                        .surface_vertices()
                        .each_ref_ext()
                        .map(|vertex| vertex.global_form().clone()),
                );

            global_vertices_from_vertices.access_in_normalized_order()
        };
        let global_vertices_from_global_form = half_edge
            .global_form()
            .vertices()
            .access_in_normalized_order();

        let ids_from_vertices = global_vertices_from_vertices
            .each_ref_ext()
            .map(|global_vertex| global_vertex.id());
        let ids_from_global_form = global_vertices_from_global_form
            .each_ref_ext()
            .map(|global_vertex| global_vertex.id());

        if ids_from_vertices != ids_from_global_form {
            errors.push(
                Box::new(Self::GlobalVertexMismatch {
                    global_vertices_from_vertices,
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

        for vertex in half_edge.surface_vertices() {
            let surface_form_surface = vertex.surface();

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
    }

    fn check_vertex_positions(
        half_edge: &HalfEdge,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let [back_position, front_position] = half_edge
            .vertices()
            .each_ref_ext()
            .map(|vertex| vertex.position());

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

    fn check_position(
        half_edge: &HalfEdge,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        for vertex in half_edge.vertices() {
            let curve_position_on_surface = half_edge
                .curve()
                .path()
                .point_from_path_coords(vertex.position());
            let surface_position = vertex.surface_form().position();

            let distance =
                curve_position_on_surface.distance_to(&surface_position);

            if distance > config.identical_max_distance {
                errors.push(
                    Box::new(Self::VertexPositionMismatch {
                        vertex: vertex.clone(),
                        curve_position_on_surface,
                        distance,
                    })
                    .into(),
                );
            }
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
        partial::{
            FullToPartialCache, Partial, PartialHalfEdge, PartialObject,
            PartialVertex,
        },
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
        let invalid =
            HalfEdge::new(valid.curve().clone(), valid.vertices().clone(), {
                let mut tmp = Partial::from(valid.global_form().clone());
                tmp.write().curve =
                    Partial::from(GlobalCurve.insert(&mut services.objects));
                tmp.build(&mut services.objects)
            });

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
        let invalid =
            HalfEdge::new(valid.curve().clone(), valid.vertices().clone(), {
                let mut tmp = Partial::from(valid.global_form().clone());
                tmp.write().vertices = valid
                    .global_form()
                    .vertices()
                    .access_in_normalized_order()
                    // Creating equal but not identical vertices here.
                    .map(|vertex| {
                        Partial::from_partial(
                            Partial::from(vertex).read().clone(),
                        )
                    });
                tmp.build(&mut services.objects)
            });

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
            let vertices = valid.vertices().clone().map(|vertex| {
                let mut vertex = PartialVertex::from_full(
                    &vertex,
                    &mut FullToPartialCache::default(),
                );
                vertex.surface_form.write().surface =
                    Partial::from(services.objects.surfaces.xz_plane());

                vertex.build(&mut services.objects)
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
            let vertices = valid.vertices().each_ref_ext().map(|vertex| {
                let mut vertex = PartialVertex::from_full(
                    vertex,
                    &mut FullToPartialCache::default(),
                );
                vertex.position = Some(Point::from([0.]));

                vertex.build(&mut services.objects)
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
    fn vertex_position_mismatch() -> anyhow::Result<()> {
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
            let vertices = valid.vertices().clone().map(|vertex| {
                let mut vertex = PartialVertex::from_full(
                    &vertex,
                    &mut FullToPartialCache::default(),
                );
                vertex.position = Some(Point::from([2.]));

                vertex.build(&mut services.objects)
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
