use fj_math::Point;

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

    /// Add a new half-edge that starts at the provided point
    ///
    /// Opens the cycle between the last and first edge, updates the last edge
    /// to go the provided point, and adds a new half-edge from the provided
    /// point the the first edge.
    ///
    /// If the cycle doesn't have any edges yet, the new edge connects to
    /// itself, starting and ending at the provided point.
    fn add_half_edge_from_global_point_to_start(
        &mut self,
        point: impl Into<Point<3>>,
    ) -> Partial<HalfEdge>;

    /// Update cycle as a polygon from the provided points
    fn update_as_polygon_from_points<O, P>(
        &mut self,
        points: O,
    ) -> O::SameSize<Partial<HalfEdge>>
    where
        O: ObjectArgument<P>,
        P: Into<Point<2>>;

    /// Update cycle as a polygon
    ///
    /// Will update each half-edge in the cycle to be a line segment.
    fn update_as_polygon(&mut self);

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

            let mut last_half_edge = last_half_edge.write();

            last_half_edge.end_vertex = shared_surface_vertex;
            last_half_edge.infer_global_form();
        }

        {
            let shared_surface_vertex =
                first_half_edge.read().start_vertex.clone();

            let mut new_half_edge = new_half_edge.write();

            new_half_edge.end_vertex = shared_surface_vertex;
            new_half_edge.infer_global_form();
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

    fn add_half_edge_from_global_point_to_start(
        &mut self,
        point: impl Into<Point<3>>,
    ) -> Partial<HalfEdge> {
        let mut half_edge = self.add_half_edge();
        half_edge
            .write()
            .start_vertex
            .write()
            .global_form
            .write()
            .position = Some(point.into());
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
        self.update_as_polygon();
        half_edges
    }

    fn update_as_polygon(&mut self) {
        for half_edge in &mut self.half_edges {
            half_edge.write().update_as_line_segment();
        }
    }

    fn connect_to_closed_edges<O>(
        &mut self,
        edges: O,
        surface: &SurfaceGeometry,
    ) -> O::SameSize<Partial<HalfEdge>>
    where
        O: ObjectArgument<Partial<HalfEdge>>,
    {
        edges.map(|other| {
            let mut this = self.add_half_edge();
            this.write().update_from_other_edge(&other, surface);
            this
        })
    }

    fn infer_vertex_positions_if_necessary(
        &mut self,
        surface: &SurfaceGeometry,
    ) {
        for half_edge in &mut self.half_edges {
            half_edge
                .write()
                .infer_vertex_positions_if_necessary(surface);
        }
    }
}
