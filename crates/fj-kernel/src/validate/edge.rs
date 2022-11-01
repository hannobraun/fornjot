use std::convert::Infallible;

use fj_interop::ext::ArrayExt;
use fj_math::Point;

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge,
        VerticesInNormalizedOrder,
    },
    storage::Handle,
};

use super::{Validate2, ValidationConfig};

impl Validate2 for HalfEdge {
    type Error = HalfEdgeValidationError;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        HalfEdgeValidationError::check_curve_identity(self)?;
        HalfEdgeValidationError::check_global_curve_identity(self)?;
        HalfEdgeValidationError::check_global_vertex_identity(self)?;
        HalfEdgeValidationError::check_vertex_positions(self)?;
        Ok(())
    }
}

impl Validate2 for GlobalEdge {
    type Error = Infallible;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// [`HalfEdge`] validation failed
#[derive(Debug, thiserror::Error)]
pub enum HalfEdgeValidationError {
    /// [`HalfEdge`] vertices are not defined on the same `Curve`
    #[error(
        "`HalfEdge` vertices are not defined on the same `Curve`\n\
        - `Curve` of back vertex: {:?}\n\
        - `Curve` of front vertex: {:?}",
        .back_curve.full_debug(),
        .front_curve.full_debug(),
    )]
    CurveMismatch {
        /// The curve of the [`HalfEdge`]'s back vertex
        back_curve: Handle<Curve>,

        /// The curve of the [`HalfEdge`]'s front vertex
        front_curve: Handle<Curve>,
    },

    /// [`HalfEdge`]'s [`GlobalCurve`]s do not match
    #[error(
        "Global form of `HalfEdge`'s `Curve` does not match `GlobalCurve` of \n\
        the `HalfEdge`'s `GlobalEdge`\n\
        - `GlobalCurve` from `Curve`: {:?}\n\
        - `GlobalCurve` from `GlobalEdge`: {:?}",
        .global_curve_from_curve.full_debug(),
        .global_curve_from_global_form.full_debug(),
    )]
    GlobalCurveMismatch {
        /// The [`GlobalCurve`] from the [`HalfEdge`]'s [`Curve`]
        global_curve_from_curve: Handle<GlobalCurve>,

        /// The [`GlobalCurve`] from the [`HalfEdge`]'s global form
        global_curve_from_global_form: Handle<GlobalCurve>,
    },

    /// [`HalfEdge`]'s [`GlobalVertex`] objects do not match
    #[error(
        "Global forms of `HalfEdge` vertices do not match vertices of \n\
        `HalfEdge`'s global form\n\
        - `GlobalVertex` objects from `Vertex` objects: {:?}\n\
        - `GlobalVertex` objects from `GlobalEdge`: {:?}",
        .global_vertices_from_vertices
            .each_ref_ext()
            .map(|vertex| vertex.full_debug()),
        .global_vertices_from_global_form
            .each_ref_ext()
            .map(|vertex| vertex.full_debug()),
    )]
    GlobalVertexMismatch {
        /// The [`GlobalVertex`] from the [`HalfEdge`]'s vertices
        global_vertices_from_vertices: [Handle<GlobalVertex>; 2],

        /// The [`GlobalCurve`] from the [`HalfEdge`]'s global form
        global_vertices_from_global_form: [Handle<GlobalVertex>; 2],
    },

    /// [`HalfEdge`]'s vertices are coincident
    #[error(
        "Vertices on curve are coincident\n\
        - Position of back vertex: {back_position:?}\n\
        - Position of front vertex: {front_position:?}"
    )]
    VerticesAreCoincident {
        /// The position of the back vertex
        back_position: Point<1>,

        /// The position of the front vertex
        front_position: Point<1>,
    },
}

impl HalfEdgeValidationError {
    fn check_curve_identity(half_edge: &HalfEdge) -> Result<(), Self> {
        let back_curve = half_edge.back().curve();
        let front_curve = half_edge.front().curve();

        if back_curve.id() != front_curve.id() {
            return Err(HalfEdgeValidationError::CurveMismatch {
                back_curve: back_curve.clone(),
                front_curve: front_curve.clone(),
            });
        }

        Ok(())
    }

    fn check_global_curve_identity(half_edge: &HalfEdge) -> Result<(), Self> {
        let global_curve_from_curve = half_edge.curve().global_form();
        let global_curve_from_global_form = half_edge.global_form().curve();

        if global_curve_from_curve.id() != global_curve_from_global_form.id() {
            return Err(Self::GlobalCurveMismatch {
                global_curve_from_curve: global_curve_from_curve.clone(),
                global_curve_from_global_form: global_curve_from_global_form
                    .clone(),
            });
        }

        Ok(())
    }

    fn check_global_vertex_identity(half_edge: &HalfEdge) -> Result<(), Self> {
        let global_vertices_from_vertices = {
            let (global_vertices_from_vertices, _) =
                VerticesInNormalizedOrder::new(
                    half_edge
                        .vertices()
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
            return Err(Self::GlobalVertexMismatch {
                global_vertices_from_vertices,
                global_vertices_from_global_form,
            });
        }

        Ok(())
    }

    fn check_vertex_positions(half_edge: &HalfEdge) -> Result<(), Self> {
        let back_position = half_edge.back().position();
        let front_position = half_edge.front().position();

        if back_position == front_position {
            return Err(Self::VerticesAreCoincident {
                back_position,
                front_position,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use fj_interop::ext::ArrayExt;

    use crate::{
        objects::{Curve, GlobalCurve, GlobalEdge, HalfEdge, Objects, Vertex},
        partial::HasPartial,
        validate::Validate2,
    };

    #[test]
    fn half_edge_curve_mismatch() -> anyhow::Result<()> {
        let valid = {
            let objects = Objects::new();

            let curve = Curve::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .as_line_from_points([[0., 0.], [1., 0.]])
                .build(&objects)?;

            let vertices = {
                [0., 1.].try_map_ext(|position| {
                    Vertex::partial()
                        .with_position(Some([position]))
                        .with_curve(Some(curve.clone()))
                        .build(&objects)
                })?
            };

            let global_form = GlobalEdge::partial()
                .with_curve(Some(curve.global_form().clone()))
                .with_vertices(Some(
                    vertices
                        .each_ref_ext()
                        .map(|vertex| vertex.global_form().clone()),
                ))
                .build(&objects)?;

            HalfEdge::new(vertices, global_form)
        };
        let invalid = {
            let objects = Objects::new();

            let curve = Curve::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .as_line_from_points([[0., 0.], [1., 0.]])
                .build(&objects)?;

            let vertices = [
                Vertex::partial()
                    .with_position(Some([0.]))
                    .with_curve(Some(curve.clone()))
                    .build(&objects)?,
                Vertex::partial()
                    .with_position(Some([1.]))
                    // Arranging for an equal but not identical curve here.
                    .with_curve(Some(curve.to_partial()))
                    .build(&objects)?,
            ];

            let global_form = GlobalEdge::partial()
                .with_curve(Some(curve.global_form().clone()))
                .with_vertices(Some(
                    vertices
                        .each_ref_ext()
                        .map(|vertex| vertex.global_form().clone()),
                ))
                .build(&objects)?;

            HalfEdge::new(vertices, global_form)
        };

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());

        Ok(())
    }

    #[test]
    fn half_edge_global_curve_mismatch() -> anyhow::Result<()> {
        let valid = {
            let objects = Objects::new();

            let curve = Curve::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .as_line_from_points([[0., 0.], [1., 0.]])
                .build(&objects)?;

            let vertices = {
                [0., 1.].try_map_ext(|position| {
                    Vertex::partial()
                        .with_position(Some([position]))
                        .with_curve(Some(curve.clone()))
                        .build(&objects)
                })?
            };

            let global_form = GlobalEdge::partial()
                .with_curve(Some(curve.global_form().clone()))
                .with_vertices(Some(
                    vertices
                        .each_ref_ext()
                        .map(|vertex| vertex.global_form().clone()),
                ))
                .build(&objects)?;

            HalfEdge::new(vertices, global_form)
        };
        let invalid = {
            let objects = Objects::new();

            let curve = Curve::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .as_line_from_points([[0., 0.], [1., 0.]])
                .build(&objects)?;

            let vertices = {
                [0., 1.].try_map_ext(|position| {
                    Vertex::partial()
                        .with_position(Some([position]))
                        .with_curve(Some(curve.clone()))
                        .build(&objects)
                })?
            };

            let global_form = GlobalEdge::partial()
                // Creating a different `GlobalCurve` here.
                .with_curve(Some(objects.global_curves.insert(GlobalCurve)?))
                .with_vertices(Some(
                    vertices
                        .each_ref_ext()
                        .map(|vertex| vertex.global_form().clone()),
                ))
                .build(&objects)?;

            HalfEdge::new(vertices, global_form)
        };

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());

        Ok(())
    }

    #[test]
    fn half_edge_global_vertex_mismatch() -> anyhow::Result<()> {
        let valid = {
            let objects = Objects::new();

            let curve = Curve::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .as_line_from_points([[0., 0.], [1., 0.]])
                .build(&objects)?;

            let vertices = {
                [0., 1.].try_map_ext(|position| {
                    Vertex::partial()
                        .with_position(Some([position]))
                        .with_curve(Some(curve.clone()))
                        .build(&objects)
                })?
            };

            let global_form = GlobalEdge::partial()
                .with_curve(Some(curve.global_form().clone()))
                .with_vertices(Some(
                    vertices
                        .each_ref_ext()
                        .map(|vertex| vertex.global_form().clone()),
                ))
                .build(&objects)?;

            HalfEdge::new(vertices, global_form)
        };
        let invalid = {
            let objects = Objects::new();

            let curve = Curve::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .as_line_from_points([[0., 0.], [1., 0.]])
                .build(&objects)?;

            let vertices = {
                [0., 1.].try_map_ext(|position| {
                    Vertex::partial()
                        .with_position(Some([position]))
                        .with_curve(Some(curve.clone()))
                        .build(&objects)
                })?
            };

            let global_form = GlobalEdge::partial()
                .with_curve(Some(curve.global_form().clone()))
                .with_vertices(Some(
                    vertices
                        .each_ref_ext()
                        // Creating different `GlobalVertex` objects here.
                        .map(|vertex| vertex.global_form().to_partial()),
                ))
                .build(&objects)?;

            HalfEdge::new(vertices, global_form)
        };

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());

        Ok(())
    }

    #[test]
    fn half_edge_vertices_are_coincident() -> anyhow::Result<()> {
        let valid = {
            let objects = Objects::new();

            let curve = Curve::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .as_line_from_points([[0., 0.], [1., 0.]])
                .build(&objects)?;

            let vertices = {
                [0., 1.].try_map_ext(|position| {
                    Vertex::partial()
                        .with_position(Some([position]))
                        .with_curve(Some(curve.clone()))
                        .build(&objects)
                })?
            };

            let global_form = GlobalEdge::partial()
                .with_curve(Some(curve.global_form().clone()))
                .with_vertices(Some(
                    vertices
                        .each_ref_ext()
                        .map(|vertex| vertex.global_form().clone()),
                ))
                .build(&objects)?;

            HalfEdge::new(vertices, global_form)
        };
        let invalid = {
            let objects = Objects::new();

            let curve = Curve::partial()
                .with_surface(Some(objects.surfaces.xy_plane()))
                .as_line_from_points([[0., 0.], [1., 0.]])
                .build(&objects)?;

            let vertices = {
                [0., 0.].try_map_ext(|position| {
                    Vertex::partial()
                        .with_position(Some([position]))
                        .with_curve(Some(curve.clone()))
                        .build(&objects)
                })?
            };

            let global_form = GlobalEdge::partial()
                .with_curve(Some(curve.global_form().clone()))
                .with_vertices(Some(
                    vertices
                        .each_ref_ext()
                        .map(|vertex| vertex.global_form().clone()),
                ))
                .build(&objects)?;

            HalfEdge::new(vertices, global_form)
        };

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());

        Ok(())
    }
}
