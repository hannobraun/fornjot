use std::convert::Infallible;

use crate::{
    objects::{Curve, GlobalCurve, GlobalEdge, HalfEdge},
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
}
