use fj_math::Point;
use itertools::Itertools;

use crate::{
    geometry::surface::SurfaceGeometry,
    objects::HalfEdge,
    partial::{Partial, PartialCycle},
};

use super::{HalfEdgeBuilder, ObjectArgument};

/// Builder API for [`PartialCycle`]
pub trait CycleBuilder {
    /// Add a new half-edge to the cycle
    ///
    /// Creates a half-edge and adds it to the cycle. The new half-edge is
    /// connected to the front vertex of the last half-edge , and the back
    /// vertex of the first edge, making sure the half-edges actually form a
    /// cycle.
    ///
    /// If this is the first half-edge being added, it is connected to itself,
    /// meaning its front and back vertices are the same.
    fn add_half_edge(&mut self) -> Partial<HalfEdge>;

    /// Add a new half-edge that starts at the provided point
    ///
    /// Opens the cycle between the last and first edge, updates the last edge
    /// to go the provided point, and adds a new half-edge from the provided
    /// point the the first edge.
    ///
    /// If the cycle doesn't have any edges yet, the new edge connects to
    /// itself, starting and ending at the provided point.
    fn add_half_edge_from_point_to_start(
        &mut self,
        point: impl Into<Point<2>>,
    ) -> Partial<HalfEdge>;

    /// Update cycle as a polygon from the provided points
    fn update_as_polygon_from_points<O, P>(
        &mut self,
        points: O,
    ) -> O::SameSize<Partial<HalfEdge>>
    where
        O: ObjectArgument<P>,
        P: Into<Point<2>>;

    /// Connect the cycles to the provided half-edges
    ///
    /// Assumes that the provided half-edges, once translated into local
    /// equivalents of this cycle, form a cycle themselves.
    ///
    /// Returns the local equivalents of the provided half-edges.
    fn connect_to_closed_edges<O>(
        &mut self,
        edges: O,
        surface: &SurfaceGeometry,
    ) -> O::SameSize<Partial<HalfEdge>>
    where
        O: ObjectArgument<Partial<HalfEdge>>;

    /// Infer the positions of all vertices, if necessary
    fn infer_vertex_positions_if_necessary(
        &mut self,
        surface: &SurfaceGeometry,
    );
}

impl CycleBuilder for PartialCycle {
    fn add_half_edge(&mut self) -> Partial<HalfEdge> {
        let mut new_half_edge = Partial::<HalfEdge>::new();

        let (first_half_edge, mut last_half_edge) =
            match self.half_edges.first() {
                Some(first_half_edge) => {
                    let first_half_edge = first_half_edge.clone();
                    let last_half_edge = self
                        .half_edges
                        .last()
                        .cloned()
                        .unwrap_or_else(|| first_half_edge.clone());

                    (first_half_edge, last_half_edge)
                }
                None => (new_half_edge.clone(), new_half_edge.clone()),
            };

        {
            let shared_surface_vertex =
                new_half_edge.read().start_vertex.clone();
            last_half_edge
                .write()
                .infer_global_form(shared_surface_vertex);
        }

        {
            let shared_surface_vertex =
                first_half_edge.read().start_vertex.clone();
            new_half_edge
                .write()
                .infer_global_form(shared_surface_vertex);
        }

        self.half_edges.push(new_half_edge.clone());
        new_half_edge
    }

    fn add_half_edge_from_point_to_start(
        &mut self,
        point: impl Into<Point<2>>,
    ) -> Partial<HalfEdge> {
        let mut half_edge = self.add_half_edge();
        half_edge.write().start_vertex.write().position = Some(point.into());
        half_edge
    }

    fn update_as_polygon_from_points<O, P>(
        &mut self,
        points: O,
    ) -> O::SameSize<Partial<HalfEdge>>
    where
        O: ObjectArgument<P>,
        P: Into<Point<2>>,
    {
        let half_edges =
            points.map(|point| self.add_half_edge_from_point_to_start(point));

        for (mut half_edge, next) in
            self.half_edges.iter().cloned().circular_tuple_windows()
        {
            half_edge.write().update_as_line_segment(next.clone());
        }

        half_edges
    }

    fn connect_to_closed_edges<O>(
        &mut self,
        edges: O,
        surface: &SurfaceGeometry,
    ) -> O::SameSize<Partial<HalfEdge>>
    where
        O: ObjectArgument<Partial<HalfEdge>>,
    {
        edges.map_with_prev(|other, prev| {
            let mut this = self.add_half_edge();
            this.write().update_from_other_edge(&other, &prev, surface);
            this
        })
    }

    fn infer_vertex_positions_if_necessary(
        &mut self,
        surface: &SurfaceGeometry,
    ) {
        for (mut half_edge, next_half_edge) in
            self.half_edges.iter().cloned().circular_tuple_windows()
        {
            let next_vertex = next_half_edge.read().start_vertex.clone();
            half_edge
                .write()
                .infer_vertex_positions_if_necessary(surface, next_vertex);
        }
    }
}
