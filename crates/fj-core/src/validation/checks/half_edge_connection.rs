use fj_math::{Point, Scalar};

use crate::{
    geometry::Geometry,
    storage::Handle,
    topology::{Cycle, Face, HalfEdge, Region, Sketch, Surface},
    validation::{validation_check::ValidationCheck, ValidationConfig},
};

/// # Adjacent [`HalfEdge`]s in [`Cycle`] are not connected
///
/// Each [`HalfEdge`] only references its start vertex. The end vertex is always
/// assumed to be the start vertex of the next [`HalfEdge`] in the cycle. This
/// part of the definition carries no redundancy, and thus doesn't need to be
/// subject to a validation check.
///
/// However, the *position* of that shared vertex is redundantly defined in both
/// [`HalfEdge`]s. This check verifies that both positions are the same.
///
/// ## Implementation Note
///
/// Having the vertex positions redundantly defined is not desirable, but
/// currently we lack the facilities to project a single definition (whether
/// local to a curve, local to a surface, or global in 3D space) into other
/// local contexts, where they are required for approximation/triangulation.
///
/// As of this writing, there is no issue for creating these facilities and
/// consolidating these redundant definitions, but the following issue tracks a
/// prerequisite of that:
///
/// <https://github.com/hannobraun/fornjot/issues/2118>
///
/// If there was a single definition for each vertex position, we wouldn't need
/// this validation check in its current form, but we would still need another
/// one that fills a similar gap. Namely, we would still need to check whether a
/// half-edge's start and end vertices are actually located on that half-edge's
/// curve.
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

impl ValidationCheck<Face> for AdjacentHalfEdgesNotConnected {
    fn check<'r>(
        object: &'r Face,
        geometry: &'r Geometry,
        config: &'r ValidationConfig,
    ) -> impl Iterator<Item = Self> + 'r {
        check_region(object.region(), object.surface(), geometry, config)
    }
}

impl ValidationCheck<Sketch> for AdjacentHalfEdgesNotConnected {
    fn check<'r>(
        object: &'r Sketch,
        geometry: &'r Geometry,
        config: &'r ValidationConfig,
    ) -> impl Iterator<Item = Self> + 'r {
        object.regions().iter().flat_map(|region| {
            check_region(region, object.surface(), geometry, config)
        })
    }
}

fn check_region<'r>(
    region: &'r Region,
    surface: &'r Handle<Surface>,
    geometry: &'r Geometry,
    config: &'r ValidationConfig,
) -> impl Iterator<Item = AdjacentHalfEdgesNotConnected> + 'r {
    [region.exterior()]
        .into_iter()
        .chain(region.interiors())
        .flat_map(|cycle| check_cycle(cycle, surface, geometry, config))
}

fn check_cycle<'r>(
    cycle: &'r Cycle,
    _: &'r Handle<Surface>,
    geometry: &'r Geometry,
    config: &'r ValidationConfig,
) -> impl Iterator<Item = AdjacentHalfEdgesNotConnected> + 'r {
    cycle.half_edges().pairs().filter_map(|(first, second)| {
        let end_pos_of_first_half_edge = {
            let [_, end] = geometry.of_half_edge(first).boundary.inner;
            geometry
                .of_half_edge(first)
                .path
                .point_from_path_coords(end)
        };
        let start_pos_of_second_half_edge =
            geometry.of_half_edge(second).start_position();

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

#[cfg(test)]
mod tests {

    use crate::{
        operations::{
            build::{BuildFace, BuildHalfEdge},
            update::{UpdateCycle, UpdateFace, UpdateRegion},
        },
        topology::{Face, HalfEdge},
        validation::ValidationCheck,
        Core,
    };

    use super::AdjacentHalfEdgesNotConnected;

    #[test]
    fn adjacent_half_edges_not_connected() -> anyhow::Result<()> {
        let mut core = Core::new();

        let surface = core.layers.topology.surfaces.space_2d();

        // We're only testing for `Face` here, not `Sketch`. Should be fine, as
        // most of the code is shared.
        let valid = Face::polygon(
            surface.clone(),
            [[0., 0.], [1., 0.], [1., 1.]],
            &mut core,
        );
        AdjacentHalfEdgesNotConnected::check_and_return_first_error(
            &valid,
            &core.layers.geometry,
        )?;

        let invalid = valid.update_region(
            |region, core| {
                region.update_exterior(
                    |cycle, core| {
                        cycle.update_half_edge(
                            cycle.half_edges().first(),
                            |_, core| {
                                [HalfEdge::line_segment(
                                    [[0., 0.], [2., 0.]],
                                    None,
                                    surface,
                                    core,
                                )]
                            },
                            core,
                        )
                    },
                    core,
                )
            },
            &mut core,
        );
        AdjacentHalfEdgesNotConnected::check_and_expect_one_error(
            &invalid,
            &core.layers.geometry,
        );

        Ok(())
    }
}
