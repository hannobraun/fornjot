use fj_interop::ext::ArrayExt;
use fj_math::{Point, Scalar};
use itertools::Itertools;

use crate::{
    objects::{Cycle, HalfEdge, SurfaceVertex},
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

    /// Mismatch between half-edge boundary and surface vertex position
    #[error(
        "Half-edge boundary on curve doesn't match surface vertex position\n\
        - Position on curve: {position_on_curve:?}\n\
        - Curve position converted to surface: {curve_position_on_surface:?}\n\
        - Surface position from vertex: {surface_position_from_vertex:?}\n\
        - Distance between the positions: {distance}\n\
        - Surface vertex: {surface_vertex:#?}\n\
        - Half-edge: {half_edge:#?}"
    )]
    HalfEdgeBoundaryMismatch {
        /// The position on the curve
        position_on_curve: Point<1>,

        /// The curve position converted into a surface position
        curve_position_on_surface: Point<2>,

        /// The surface position from the vertex
        surface_position_from_vertex: Point<2>,

        /// The distance between the positions
        distance: Scalar,

        /// The surface vertex
        surface_vertex: Handle<SurfaceVertex>,

        /// The half-edge
        half_edge: Handle<HalfEdge>,
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
        for (half_edge, next) in
            cycle.half_edges().circular_tuple_windows::<(_, _)>()
        {
            let boundary_and_vertices = half_edge
                .boundary()
                .zip_ext([half_edge.start_vertex(), next.start_vertex()]);
            for (position_on_curve, surface_vertex) in boundary_and_vertices {
                let curve_position_on_surface = half_edge
                    .curve()
                    .path()
                    .point_from_path_coords(position_on_curve);
                let surface_position_from_vertex = surface_vertex.position();

                let distance = curve_position_on_surface
                    .distance_to(&surface_position_from_vertex);

                if distance > config.identical_max_distance {
                    errors.push(
                        Self::HalfEdgeBoundaryMismatch {
                            position_on_curve,
                            curve_position_on_surface,
                            surface_position_from_vertex,
                            distance,
                            surface_vertex: surface_vertex.clone(),
                            half_edge: half_edge.clone(),
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
            let surface = services.objects.surfaces.xy_plane();

            let mut cycle = PartialCycle {
                surface: Partial::from(surface.clone()),
                ..Default::default()
            };
            cycle.update_as_polygon_from_points([[0., 0.], [1., 0.], [0., 1.]]);
            cycle.infer_vertex_positions_if_necessary(&surface.geometry());
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

            Cycle::new(valid.surface().clone(), half_edges)
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }

    #[test]
    fn vertex_position_mismatch() -> anyhow::Result<()> {
        let mut services = Services::new();

        let valid = {
            let surface = services.objects.surfaces.xy_plane();

            let mut cycle = PartialCycle {
                surface: Partial::from(surface.clone()),
                ..Default::default()
            };
            cycle.update_as_polygon_from_points([[0., 0.], [1., 0.], [0., 1.]]);
            cycle.infer_vertex_positions_if_necessary(&surface.geometry());
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

            Cycle::new(valid.surface().clone(), half_edges)
        };

        valid.validate_and_return_first_error()?;
        assert!(invalid.validate_and_return_first_error().is_err());

        Ok(())
    }
}
