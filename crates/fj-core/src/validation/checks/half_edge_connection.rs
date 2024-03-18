use fj_math::{Point, Scalar};

use crate::{
    objects::{Cycle, HalfEdge},
    storage::Handle,
    validation::{validation_check::ValidationCheck, ValidationConfig},
};

/// Adjacent [`HalfEdge`]s in [`Cycle`] are not connected
///
/// Each [`HalfEdge`] only references its start vertex. The end vertex is always
/// assumed to be the start vertex of the next [`HalfEdge`] in the cycle. This
/// part of the definition carries no redundancy, and thus doesn't need to be
/// subject to a validation check.
///
/// However, the *position* of that shared vertex is redundantly defined in both
/// [`HalfEdge`]s. This check verifies that both positions are the same.
#[derive(Clone, Debug, thiserror::Error)]
#[error(
    "Adjacent `HalfEdge`s in `Cycle` are not connected\n\
    - End position of first `HalfEdge`: {end_pos_of_first_half_edge:?}\n\
    - Start position of second `HalfEdge`: {start_pos_of_second_half_edge:?}\n\
    - Distance between vertices: {distance_between_positions}\n\
    - The unconnected `HalfEdge`s: {unconnected_half_edges:#?}"
)]
pub struct AdjacentHalfEdgesNotConnected {
    /// The end position of the first [`HalfEdge`]
    pub end_pos_of_first_half_edge: Point<2>,

    /// The start position of the second [`HalfEdge`]
    pub start_pos_of_second_half_edge: Point<2>,

    /// The distance between the two positions
    pub distance_between_positions: Scalar,

    /// The edges
    pub unconnected_half_edges: [Handle<HalfEdge>; 2],
}

impl ValidationCheck<Cycle> for AdjacentHalfEdgesNotConnected {
    fn check(
        object: &Cycle,
        config: &ValidationConfig,
    ) -> impl Iterator<Item = Self> {
        object.half_edges().pairs().filter_map(|(first, second)| {
            let end_pos_of_first_half_edge = {
                let [_, end] = first.boundary().inner;
                first.path().point_from_path_coords(end)
            };
            let start_pos_of_second_half_edge = second.start_position();

            let distance_between_positions = (end_pos_of_first_half_edge
                - start_pos_of_second_half_edge)
                .magnitude();

            if distance_between_positions > config.identical_max_distance {
                return Some(AdjacentHalfEdgesNotConnected {
                    end_pos_of_first_half_edge,
                    start_pos_of_second_half_edge,
                    distance_between_positions,
                    unconnected_half_edges: [first.clone(), second.clone()],
                });
            }

            None
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        objects::{Cycle, HalfEdge},
        operations::{
            build::{BuildCycle, BuildHalfEdge},
            update::UpdateCycle,
        },
        validation::ValidationCheck,
        Core,
    };

    use super::AdjacentHalfEdgesNotConnected;

    #[test]
    fn adjacent_half_edges_connected() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid = Cycle::polygon([[0., 0.], [1., 0.], [1., 1.]], &mut core);
        AdjacentHalfEdgesNotConnected::check_and_return_first_error(
            &valid,
            &core.layers.geometry,
        )?;

        let invalid = valid.update_half_edge(
            valid.half_edges().first(),
            |_, core| {
                [HalfEdge::line_segment([[0., 0.], [2., 0.]], None, core)]
            },
            &mut core,
        );
        AdjacentHalfEdgesNotConnected::check_and_expect_one_error(&invalid);

        Ok(())
    }
}
