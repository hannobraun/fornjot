use fj_interop::ext::ArrayExt;
use fj_math::{Point, Scalar, Winding};
use itertools::Itertools;

use crate::{
    objects::{Face, HalfEdge},
    storage::Handle,
};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Face {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        FaceValidationError::check_interior_winding(self, errors);
        FaceValidationError::check_vertex_positions(self, config, errors);
    }
}

/// [`Face`] validation error
#[derive(Clone, Debug, thiserror::Error)]
pub enum FaceValidationError {
    /// Interior of [`Face`] has invalid winding; must be opposite of exterior
    #[error(
        "Interior of `Face` has invalid winding; must be opposite of exterior\n\
        - Winding of exterior cycle: {exterior_winding:#?}\n\
        - Winding of interior cycle: {interior_winding:#?}\n\
        - `Face`: {face:#?}"
    )]
    InvalidInteriorWinding {
        /// The winding of the [`Face`]'s exterior cycle
        exterior_winding: Winding,

        /// The winding of the invalid interior cycle
        interior_winding: Winding,

        /// The face
        face: Face,
    },

    /// Mismatch between edge boundary and `GlobalVertex` positions
    #[error(
        "`HalfEdge` boundary doesn't match position of `GlobalVertex`\n\
        - Curve position: {curve_position:?}\n\
        - Curve position converted to global position: \
            {curve_position_as_global:?}\n\
        - Global position: {global_position:?}\n\
        - Distance between the positions: {distance}\n\
        - `HalfEdge`: {half_edge:#?}"
    )]
    VertexPositionMismatch {
        /// The position of the surface vertex
        curve_position: Point<1>,

        /// The surface position converted into a global position
        curve_position_as_global: Point<3>,

        /// The position of the global vertex
        global_position: Point<3>,

        /// The distance between the positions
        distance: Scalar,

        /// The half-edge
        half_edge: Handle<HalfEdge>,
    },
}

impl FaceValidationError {
    fn check_interior_winding(face: &Face, errors: &mut Vec<ValidationError>) {
        let exterior_winding = face.exterior().winding();

        for interior in face.interiors() {
            let interior_winding = interior.winding();

            if exterior_winding == interior_winding {
                errors.push(
                    Self::InvalidInteriorWinding {
                        exterior_winding,
                        interior_winding,
                        face: face.clone(),
                    }
                    .into(),
                );
            }
        }
    }

    fn check_vertex_positions(
        face: &Face,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        for cycle in face.all_cycles() {
            for (half_edge, next_half_edge) in
                cycle.half_edges().circular_tuple_windows()
            {
                for (curve_position, vertex) in half_edge.boundary().zip_ext([
                    half_edge.start_vertex(),
                    next_half_edge.start_vertex(),
                ]) {
                    let curve_position_as_surface = half_edge
                        .curve()
                        .point_from_path_coords(curve_position);
                    let curve_position_as_global = face
                        .surface()
                        .geometry()
                        .point_from_surface_coords(curve_position_as_surface);
                    let global_position = vertex.global_form().position();

                    let distance =
                        curve_position_as_global.distance_to(&global_position);

                    if distance > config.identical_max_distance {
                        errors.push(
                            Self::VertexPositionMismatch {
                                curve_position,
                                curve_position_as_global,
                                global_position,
                                distance,
                                half_edge: half_edge.clone(),
                            }
                            .into(),
                        );
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{Scalar, Vector};

    use crate::{
        algorithms::reverse::Reverse,
        builder::{CycleBuilder, FaceBuilder, HalfEdgeBuilder},
        insert::Insert,
        objects::{Cycle, Face, HalfEdge},
        partial::{Partial, PartialFace, PartialObject},
        services::Services,
        validate::{FaceValidationError, Validate, ValidationError},
    };

    #[test]
    fn face_invalid_interior_winding() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let mut face = PartialFace {
                surface: Partial::from(services.objects.surfaces.xy_plane()),
                ..Default::default()
            };
            face.exterior.write().update_as_polygon_from_points([
                [0., 0.],
                [3., 0.],
                [0., 3.],
            ]);
            face.add_interior().write().update_as_polygon_from_points([
                [1., 1.],
                [1., 2.],
                [2., 1.],
            ]);
            face.build(&mut services.objects)
        };
        let invalid = {
            let interiors = valid
                .interiors()
                .cloned()
                .map(|cycle| cycle.reverse(&mut services.objects))
                .collect::<Vec<_>>();

            Face::new(
                valid.surface().clone(),
                valid.exterior().clone(),
                interiors,
                valid.color(),
            )
        };

        valid.validate_and_return_first_error()?;
        assert!(matches!(
            invalid.validate_and_return_first_error(),
            Err(ValidationError::Face(
                FaceValidationError::InvalidInteriorWinding { .. }
            ))
        ));

        Ok(())
    }

    #[test]
    fn vertex_position_mismatch() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let surface = services.objects.surfaces.xy_plane();

            let mut face = PartialFace {
                surface: Partial::from(surface.clone()),
                ..Default::default()
            };

            let mut half_edge = face.exterior.write().add_half_edge();
            half_edge.write().update_as_circle_from_radius(1.);
            let next_vertex =
                half_edge.read().start_vertex.read().global_form.clone();
            half_edge.write().infer_vertex_positions_if_necessary(
                &surface.geometry(),
                next_vertex,
            );

            face.build(&mut services.objects)
        };
        let invalid = {
            let half_edge = {
                let half_edge = valid.exterior().half_edges().next().unwrap();

                let boundary = half_edge
                    .boundary()
                    .map(|point| point + Vector::from([Scalar::PI / 2.]));

                HalfEdge::new(
                    half_edge.curve(),
                    boundary,
                    half_edge.start_vertex().clone(),
                    half_edge.global_form().clone(),
                )
                .insert(&mut services.objects)
            };

            let exterior =
                Cycle::new([half_edge]).insert(&mut services.objects);

            Face::new(
                valid.surface().clone(),
                exterior,
                valid.interiors().cloned(),
                valid.color(),
            )
        };

        valid.validate_and_return_first_error()?;
        assert!(matches!(
            invalid.validate_and_return_first_error(),
            Err(ValidationError::Face(
                FaceValidationError::VertexPositionMismatch { .. }
            ))
        ));

        Ok(())
    }
}
