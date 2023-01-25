use fj_interop::ext::ArrayExt;
use fj_math::{Point, Scalar};
use itertools::Itertools;

use crate::{
    objects::{Cycle, SurfaceVertex},
    storage::Handle,
};

use super::{Validate, ValidationConfig, ValidationError};

impl Validate for Cycle {
    fn validate_with_config(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        CycleValidationError::check_half_edge_connections(self, errors);
        CycleValidationError::check_half_edge_boundaries(self, config, errors);

        // We don't need to check that all half-edges are defined in the same
        // surface. We already check that they are connected by identical
        // surface vertices, so that would be redundant.
    }
}

/// [`Cycle`] validation error
#[derive(Clone, Debug, thiserror::Error)]
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

    /// Mismatch between position of the vertex and position of its surface form
    #[error(
        "Half-edge boundary on curve doesn't match surface vertex position\n\
        - Position on curve: {position_on_curve:#?}\n\
        - Surface vertex: {surface_vertex:#?}\n\
        - Curve position converted to surface: {curve_position_on_surface:?}\n\
        - Distance between the positions: {distance}"
    )]
    HalfEdgeBoundaryMismatch {
        /// The position on the curve
        position_on_curve: Point<1>,

        /// The surface vertex
        surface_vertex: Handle<SurfaceVertex>,

        /// The curve position converted into a surface position
        curve_position_on_surface: Point<2>,

        /// The distance between the positions
        distance: Scalar,
    },
}

impl CycleValidationError {
    fn check_half_edge_connections(
        cycle: &Cycle,
        errors: &mut Vec<ValidationError>,
    ) {
        for (a, b) in cycle.half_edges().circular_tuple_windows() {
            let [_, prev] = a.surface_vertices();
            let [next, _] = b.surface_vertices();

            if prev.id() != next.id() {
                errors.push(
                    Self::HalfEdgeConnection {
                        prev: prev.clone(),
                        next: next.clone(),
                    }
                    .into(),
                );
            }
        }
    }

    fn check_half_edge_boundaries(
        cycle: &Cycle,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
    ) {
        for half_edge in cycle.half_edges() {
            for (position_on_curve, surface_vertex) in
                half_edge.boundary().zip_ext(half_edge.surface_vertices())
            {
                let curve_position_on_surface = half_edge
                    .curve()
                    .path()
                    .point_from_path_coords(position_on_curve);
                let surface_position = surface_vertex.position();

                let distance =
                    curve_position_on_surface.distance_to(&surface_position);

                if distance > config.identical_max_distance {
                    errors.push(
                        Self::HalfEdgeBoundaryMismatch {
                            position_on_curve,
                            surface_vertex: surface_vertex.clone(),
                            curve_position_on_surface,
                            distance,
                        }
                        .into(),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use fj_math::Point;

    use crate::{
        builder::CycleBuilder,
        objects::Cycle,
        partial::{Partial, PartialCycle, PartialObject},
        services::Services,
        validate::Validate,
    };

    #[test]
    fn cycle_half_edge_connections() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let mut cycle = PartialCycle {
                surface: Partial::from(services.objects.surfaces.xy_plane()),
                ..Default::default()
            };
            cycle.update_as_polygon_from_points([[0., 0.], [1., 0.], [0., 1.]]);
            cycle.build(&mut services.objects)
        };
        let invalid = {
            let mut half_edges = valid
                .half_edges()
                .map(|half_edge| Partial::from(half_edge.clone()))
                .collect::<Vec<_>>();

            // Sever connection between the last and first half-edge in the
            // cycle.
            {
                let first_half_edge = half_edges.first_mut().unwrap();
                let [first_vertex, _] = &mut first_half_edge.write().vertices;
                let surface_vertex =
                    Partial::from_partial(first_vertex.1.read().clone());
                first_vertex.1 = surface_vertex;
            }

            let half_edges = half_edges
                .into_iter()
                .map(|half_edge| half_edge.build(&mut services.objects));

            Cycle::new(half_edges)
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }

    #[test]
    fn vertex_position_mismatch() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let mut cycle = PartialCycle {
                surface: Partial::from(services.objects.surfaces.xy_plane()),
                ..Default::default()
            };
            cycle.update_as_polygon_from_points([[0., 0.], [1., 0.], [0., 1.]]);
            cycle.build(&mut services.objects)
        };
        let invalid = {
            let mut half_edges = valid
                .half_edges()
                .map(|half_edge| Partial::from(half_edge.clone()))
                .collect::<Vec<_>>();

            // Update a single boundary position so it becomes wrong.
            if let Some(half_edge) = half_edges.first_mut() {
                half_edge.write().vertices[0].0.replace(Point::from([-1.]));
            }

            let half_edges = half_edges
                .into_iter()
                .map(|half_edge| half_edge.build(&mut services.objects));

            Cycle::new(half_edges)
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }
}
