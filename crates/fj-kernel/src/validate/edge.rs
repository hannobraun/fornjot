use fj_math::{Point, Scalar};

use crate::objects::{GlobalEdge, HalfEdge};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for HalfEdge {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        HalfEdgeValidationError::check_vertex_coincidence(self, config, errors);
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
    fn check_vertex_coincidence(
        half_edge: &HalfEdge,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let [back_position, front_position] = half_edge.boundary();
        let distance = (back_position - front_position).magnitude();

        if distance < config.distinct_min_distance {
            errors.push(
                Self::VerticesAreCoincident {
                    back_position,
                    front_position,
                    distance,
                    half_edge: half_edge.clone(),
                }
                .into(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use fj_math::Point;

    use crate::{
        builder::{CycleBuilder, HalfEdgeBuilder},
        objects::HalfEdge,
        partial::{PartialCycle, PartialObject},
        services::Services,
        validate::{HalfEdgeValidationError, Validate, ValidationError},
    };

    #[test]
    fn half_edge_vertices_are_coincident() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let surface = services.objects.surfaces.xy_plane();

            let mut cycle = PartialCycle::new();

            let [mut half_edge, next_half_edge, _] = cycle
                .update_as_polygon_from_points([[0., 0.], [1., 0.], [1., 1.]]);
            half_edge.write().infer_vertex_positions_if_necessary(
                &surface.geometry(),
                next_half_edge.read().start_vertex.clone(),
            );

            half_edge.build(&mut services.objects)
        };
        let invalid = {
            let boundary = [Point::from([0.]); 2];

            HalfEdge::new(
                valid.curve(),
                boundary,
                valid.start_vertex().clone(),
                valid.global_form().clone(),
            )
        };

        valid.validate_and_return_first_error()?;
        assert!(matches!(
            invalid.validate_and_return_first_error(),
            Err(ValidationError::HalfEdge(
                HalfEdgeValidationError::VerticesAreCoincident { .. }
            ))
        ));

        Ok(())
    }
}
