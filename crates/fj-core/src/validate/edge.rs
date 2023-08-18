use fj_math::{Point, Scalar};

use crate::objects::Edge;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Edge {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        EdgeValidationError::check_vertex_coincidence(self, config, errors);
    }
}

/// [`Edge`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum EdgeValidationError {
    /// [`Edge`]'s vertices are coincident
    #[error(
        "Vertices of `Edge` on curve are coincident\n\
        - Position of back vertex: {back_position:?}\n\
        - Position of front vertex: {front_position:?}\n\
        - `Edge`: {half_edge:#?}"
    )]
    VerticesAreCoincident {
        /// The position of the back vertex
        back_position: Point<1>,

        /// The position of the front vertex
        front_position: Point<1>,

        /// The distance between the two vertices
        distance: Scalar,

        /// The half-edge
        half_edge: Edge,
    },
}

impl EdgeValidationError {
    fn check_vertex_coincidence(
        half_edge: &Edge,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        let [back_position, front_position] = half_edge.boundary().inner;
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
        assert_contains_err,
        objects::Edge,
        operations::BuildEdge,
        services::Services,
        validate::{EdgeValidationError, Validate, ValidationError},
    };

    #[test]
    fn half_edge_vertices_are_coincident() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid =
            Edge::line_segment([[0., 0.], [1., 0.]], None, &mut services);
        let invalid = {
            let boundary = [Point::from([0.]); 2];

            Edge::new(
                valid.path(),
                boundary,
                valid.curve().clone(),
                valid.start_vertex().clone(),
            )
        };

        valid.validate_and_return_first_error()?;
        assert_contains_err!(
            invalid,
            ValidationError::HalfEdge(
                EdgeValidationError::VerticesAreCoincident { .. }
            )
        );

        Ok(())
    }
}
