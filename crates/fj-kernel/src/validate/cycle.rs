use crate::objects::Cycle;
use crate::objects::HalfEdge;
use fj_math::Point;
use fj_math::Scalar;
use itertools::Itertools;

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Cycle {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        CycleValidationError::check_half_edges_disconnected(
            self, config, errors,
        )
    }
}

/// [`Cycle`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum CycleValidationError {
    /// [`Cycle`]'s half-edges are not connected
    #[error(
        "Adjacent `HalfEdge`s are distinct\n\
        - End position of first `HalfEdge`: {end_of_first:?}\n\
        - Start position of second `HalfEdge`: {start_of_second:?}\n\
        - `HalfEdge`s: {half_edges:#?}"
    )]
    HalfEdgesDisconnected {
        /// The end position of the first [`HalfEdge`]
        end_of_first: Point<2>,

        /// The start position of the second [`HalfEdge`]
        start_of_second: Point<2>,

        /// The distance between the two vertices
        distance: Scalar,

        /// The half-edge
        half_edges: Box<(HalfEdge, HalfEdge)>,
    },
    #[error("Expected at least one `HalfEdge`\n")]
    NotEnoughHalfEdges,
}

impl CycleValidationError {
    fn check_half_edges_disconnected(
        cycle: &Cycle,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        // If there are no half edges
        if cycle.half_edges().next().is_none() {
            errors.push(Self::NotEnoughHalfEdges.into());
            return;
        }
        for (first, second) in cycle
            .half_edges()
            // Chain the first half_edge so that we make sure that the last connects to the first.
            // This unwrap will never fail because we checked before that there are enough half_edges.
            .chain(std::iter::once(cycle.half_edges().next().unwrap()))
            .tuple_windows()
        {
            let end_of_first = first.end_position();
            let start_of_second = second.start_position();

            let distance = (end_of_first - start_of_second).magnitude();

            if distance > config.identical_max_distance {
                errors.push(
                    Self::HalfEdgesDisconnected {
                        end_of_first,
                        start_of_second,
                        distance,
                        half_edges: Box::new((
                            first.clone_object(),
                            second.clone_object(),
                        )),
                    }
                    .into(),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        builder::{CycleBuilder, HalfEdgeBuilder},
        objects::Cycle,
        services::Services,
        validate::{cycle::CycleValidationError, Validate, ValidationError},
    };

    #[test]
    fn half_edges_connected() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = Cycle::new([])
            .update_as_polygon_from_points(
                [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]],
                &mut services.objects,
            )
            .0;

        valid.validate_and_return_first_error()?;

        let first = HalfEdgeBuilder::line_segment([[0., 0.], [1., 0.]], None)
            .build(&mut services.objects);
        let second = HalfEdgeBuilder::line_segment([[0., 0.], [1., 0.]], None)
            .build(&mut services.objects);

        let invalid = Cycle::new([])
            .add_half_edge(first, &mut services.objects)
            .0
            .add_half_edge(second, &mut services.objects)
            .0;

        assert!(matches!(
            invalid.validate_and_return_first_error(),
            Err(ValidationError::Cycle(
                CycleValidationError::HalfEdgesDisconnected { .. }
            ))
        ));

        Ok(())
    }
}
