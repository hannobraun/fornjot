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
        );
        CycleValidationError::check_enough_half_edges(self, config, errors);
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
    /// [`Cycle`]'s should have at least one `HalfEdge`
    #[error("Expected at least one `HalfEdge`\n")]
    NotEnoughHalfEdges,
}

impl CycleValidationError {
    fn check_enough_half_edges(
        cycle: &Cycle,
        _config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        // If there are no half edges
        if cycle.half_edges().next().is_none() {
            errors.push(Self::NotEnoughHalfEdges.into());
        }
    }

    fn check_half_edges_disconnected(
        cycle: &Cycle,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        for (first, second) in cycle.half_edges().circular_tuple_windows() {
            let end_of_first = {
                let [_, end] = first.boundary();
                first.curve().point_from_path_coords(end)
            };
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
        assert_contains_err,
        builder::CycleBuilder,
        objects::{Cycle, HalfEdge},
        operations::{BuildCycle, BuildHalfEdge, Insert, UpdateCycle},
        services::Services,
        validate::{cycle::CycleValidationError, Validate, ValidationError},
    };

    #[test]
    fn half_edges_connected() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = CycleBuilder::polygon(
            [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]],
            &mut services,
        )
        .build(&mut services);

        valid.validate_and_return_first_error()?;

        let disconnected = {
            let half_edges = [
                HalfEdge::line_segment(
                    [[0., 0.], [1., 0.]],
                    None,
                    &mut services,
                ),
                HalfEdge::line_segment(
                    [[0., 0.], [1., 0.]],
                    None,
                    &mut services,
                ),
            ];
            let half_edges = half_edges
                .map(|half_edge| half_edge.insert(&mut services.objects));

            Cycle::empty().add_half_edges(half_edges)
        };

        assert_contains_err!(
            disconnected,
            ValidationError::Cycle(
                CycleValidationError::HalfEdgesDisconnected { .. }
            )
        );

        let empty = Cycle::new([]);
        assert_contains_err!(
            empty,
            ValidationError::Cycle(CycleValidationError::NotEnoughHalfEdges)
        );
        Ok(())
    }
}
