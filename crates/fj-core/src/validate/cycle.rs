use fj_math::{Point, Scalar};

use crate::{
    objects::{Cycle, HalfEdge},
    storage::Handle,
    validation::{ValidationConfig, ValidationError},
};

use super::Validate;

impl Validate for Cycle {
    fn validate(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        CycleValidationError::check_half_edge_connections(self, config, errors);
    }
}

/// [`Cycle`] validation failed
#[derive(Clone, Debug, thiserror::Error)]
pub enum CycleValidationError {
    /// [`Cycle`]'s edges are not connected
    #[error(
        "Adjacent `HalfEdge`s are not connected\n\
        - End position of first `HalfEdge`: {end_of_first:?}\n\
        - Start position of second `HalfEdge`: {start_of_second:?}\n\
        - Distance between vertices: {distance}\n\
        - `HalfEdge`s: {half_edges:#?}"
    )]
    HalfEdgesNotConnected {
        /// The end position of the first [`HalfEdge`]
        end_of_first: Point<2>,

        /// The start position of the second [`HalfEdge`]
        start_of_second: Point<2>,

        /// The distance between the two vertices
        distance: Scalar,

        /// The edges
        half_edges: [Handle<HalfEdge>; 2],
    },
}

impl CycleValidationError {
    fn check_half_edge_connections(
        cycle: &Cycle,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        for (first, second) in cycle.half_edges().pairs() {
            let end_of_first = {
                let [_, end] = first.boundary().inner;
                first.path().point_from_path_coords(end)
            };
            let start_of_second = second.start_position();

            let distance = (end_of_first - start_of_second).magnitude();

            if distance > config.identical_max_distance {
                errors.push(
                    Self::HalfEdgesNotConnected {
                        end_of_first,
                        start_of_second,
                        distance,
                        half_edges: [first.clone(), second.clone()],
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
        objects::{Cycle, HalfEdge},
        operations::{
            build::{BuildCycle, BuildHalfEdge},
            update::UpdateCycle,
        },
        validate::{cycle::CycleValidationError, Validate},
        validation::ValidationError,
        Core,
    };

    #[test]
    fn edges_connected() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid =
            Cycle::polygon([[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]], &mut core);

        valid.validate_and_return_first_error()?;

        let disconnected = {
            let edges = [
                HalfEdge::line_segment([[0., 0.], [1., 0.]], None, &mut core),
                HalfEdge::line_segment([[0., 0.], [1., 0.]], None, &mut core),
            ];

            Cycle::empty().add_half_edges(edges, &mut core)
        };

        assert_contains_err!(
            disconnected,
            ValidationError::Cycle(
                CycleValidationError::HalfEdgesNotConnected { .. }
            )
        );

        Ok(())
    }
}
