use itertools::Itertools;

use crate::{
    objects::{Cycle, SurfaceVertex},
    storage::Handle,
};

use super::{Validate, ValidationConfig};

impl Validate for Cycle {
    type Error = CycleValidationError;

    fn validate_with_config(
        &self,
        _: &ValidationConfig,
    ) -> Result<(), Self::Error> {
        CycleValidationError::check_half_edge_connections(self)?;

        // We don't need to check that all half-edges are defined in the same
        // surface. We already check that they are connected by identical
        // surface vertices, so that would be redundant.

        Ok(())
    }
}

/// [`Cycle`] validation error
#[derive(Debug, thiserror::Error)]
pub enum CycleValidationError {
    /// Half-edges are not connected
    #[error(
        "`HalfEdge`s of `Cycle` are not connected\n\
        - Front vertex of previous `HalfEdge`: {prev:#?}\n\
        - Back vertex of next `HalfEdge`: {next:#?}"
    )]
    HalfEdgeConnection {
        /// The front vertex of the previous half-edge
        prev: Handle<SurfaceVertex>,

        /// The back vertex of the next half-edge
        next: Handle<SurfaceVertex>,
    },
}

impl CycleValidationError {
    fn check_half_edge_connections(cycle: &Cycle) -> Result<(), Self> {
        for (a, b) in cycle.half_edges().circular_tuple_windows() {
            let [_, prev] = a.vertices();
            let [next, _] = b.vertices();

            let prev = prev.surface_form();
            let next = next.surface_form();

            if prev.id() != next.id() {
                return Err(Self::HalfEdgeConnection {
                    prev: prev.clone(),
                    next: next.clone(),
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        builder::{CycleBuilder, HalfEdgeBuilder, VertexBuilder},
        insert::Insert,
        objects::{Cycle, Objects},
        partial::HasPartial,
        validate::{Validate, ValidationError},
    };

    #[test]
    fn cycle_half_edge_connections() -> anyhow::Result<()> {
        let mut objects = Objects::new();

        let valid = Cycle::partial()
            .with_poly_chain_from_points(
                objects.surfaces.xy_plane(),
                [[0., 0.], [1., 0.], [0., 1.]],
            )
            .close_with_line_segment()
            .build(&mut objects)?;
        let invalid = {
            let mut half_edges = valid
                .half_edges()
                .map(|half_edge| half_edge.to_partial())
                .collect::<Vec<_>>();

            let first_half_edge = &mut half_edges[0];
            let [first_vertex, _] = first_half_edge.vertices.clone();

            // Sever connection between the last and first half-edge in the
            // cycle.
            let mut first_vertex = first_vertex.into_partial();
            first_vertex.infer_surface_form();
            *first_half_edge = first_half_edge
                .clone()
                .with_back_vertex(first_vertex)
                .infer_global_form();

            let half_edges = half_edges
                .into_iter()
                .map(|half_edge| -> anyhow::Result<_, ValidationError> {
                    Ok(half_edge.build(&mut objects)?.insert(&mut objects)?)
                })
                .collect::<Result<Vec<_>, _>>()?;

            Cycle::new(half_edges)
        };

        assert!(valid.validate().is_ok());
        assert!(invalid.validate().is_err());

        Ok(())
    }
}
