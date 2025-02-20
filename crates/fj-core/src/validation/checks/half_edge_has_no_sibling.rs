use std::collections::BTreeMap;

use crate::{
    geometry::Geometry,
    queries::{BoundingVerticesOfHalfEdge, SiblingOfHalfEdge},
    storage::Handle,
    topology::{HalfEdge, Shell},
    validation::{ValidationCheck, ValidationConfig},
};

/// A [`Shell`] contains a [`HalfEdge`] without a sibling
///
/// Half-edges that are coincident must reference the same curve. This makes
/// those half-edges siblings.
///
/// In a shell, every half-edge must have a sibling. If that is not the case,
/// this is a sign of either of the following:
/// - That the shell is not closed, meaning it has some kind of hole.
/// - If the shell is closed, that its topological object graph is not valid.
#[derive(Clone, Debug, thiserror::Error)]
#[error("Half-edge has no sibling: {half_edge:#?}")]
pub struct HalfEdgeHasNoSibling {
    /// The half-edge that does not have a sibling
    pub half_edge: Handle<HalfEdge>,
}

impl ValidationCheck<Shell> for HalfEdgeHasNoSibling {
    fn check<'r>(
        object: &'r Shell,
        _: &'r Geometry,
        _: &'r ValidationConfig,
    ) -> impl Iterator<Item = Self> + 'r {
        let mut unmatched_half_edges = BTreeMap::new();

        for face in object.faces() {
            for cycle in face.region().all_cycles() {
                for half_edge in cycle.half_edges() {
                    let curve = half_edge.curve().clone();
                    let vertices =
                        cycle.bounding_vertices_of_half_edge(half_edge).expect(
                            "`half_edge` came from `cycle`, must exist there",
                        );

                    let key = (curve.clone(), vertices.clone());
                    let key_reversed = (curve, vertices.reverse());

                    match unmatched_half_edges.remove(&key_reversed) {
                        Some(sibling) => {
                            // This must be the sibling of the half-edge we're
                            // currently looking at. Let's make sure the logic
                            // we use here to determine that matches the
                            // "official" definition.
                            assert!(object.are_siblings(half_edge, sibling));
                        }
                        None => {
                            // If this half-edge has a sibling, we haven't seen
                            // it yet. Let's store this half-edge then, in case
                            // we come across the sibling later.
                            unmatched_half_edges.insert(key, half_edge);
                        }
                    }
                }
            }
        }

        unmatched_half_edges
            .into_values()
            .cloned()
            .map(|half_edge| HalfEdgeHasNoSibling { half_edge })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Core,
        operations::{build::BuildShell, update::UpdateShell},
        topology::Shell,
        validation::{ValidationCheck, checks::HalfEdgeHasNoSibling},
    };

    #[test]
    fn half_edge_has_no_sibling() -> anyhow::Result<()> {
        let mut core = Core::new();

        let valid = Shell::tetrahedron(
            [[0., 0., 0.], [0., 1., 0.], [1., 0., 0.], [0., 0., 1.]],
            &mut core,
        );
        HalfEdgeHasNoSibling::check_and_return_first_error(
            &valid.shell,
            &core.layers.geometry,
        )?;

        let invalid = valid.shell.remove_face(&valid.abc.face);
        assert!(
            HalfEdgeHasNoSibling::check_and_return_first_error(
                &invalid,
                &core.layers.geometry,
            )
            .is_err()
        );

        Ok(())
    }
}
