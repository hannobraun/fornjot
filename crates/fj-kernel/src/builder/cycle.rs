use std::collections::VecDeque;

use fj_interop::ext::ArrayExt;
use fj_math::Point;

use crate::{
    builder::SurfaceBuilder,
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

    /// Update cycle as a triangle, from global (3D) points
    ///
    /// Uses the three points to infer a plane that is used as the surface.
    ///
    /// # Implementation Note
    ///
    /// This method is probably just temporary, and will be generalized into a
    /// "update as polygon from global points" method sooner or later. For now,
    /// I didn't want to deal with the question of how to infer the surface, and
    /// how to handle points that don't fit that surface.
    fn update_as_triangle_from_global_points(
        &mut self,
        points: [impl Into<Point<3>>; 3],
    ) -> [Partial<HalfEdge>; 3];

    /// Connect the cycle to the provided half-edges
    ///
    /// Assumes that the provided half-edges, once translated into local
    /// equivalents of this cycle, will not form a cycle themselves.
    ///
    /// Returns the local equivalents of the provided half-edges and, as the
    /// last entry, an additional half-edge that closes the cycle.
    fn connect_to_open_edges<O>(
        &mut self,
        edges: O,
    ) -> O::SizePlusOne<Partial<HalfEdge>>
    where
        O: ObjectArgument<Partial<HalfEdge>>;

    /// Connect the cycles to the provided half-edges
    ///
    /// Assumes that the provided half-edges, once translated into local
    /// equivalents of this cycle, form a cycle themselves.
    ///
    /// Returns the local equivalents of the provided half-edges.
    fn connect_to_closed_edges<O>(
        &mut self,
        edges: O,
    ) -> O::SameSize<Partial<HalfEdge>>
    where
        O: ObjectArgument<Partial<HalfEdge>>;
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
            let shared_surface_vertex = {
                let [vertex, _] = &new_half_edge.read().vertices;
                vertex.1.clone()
            };

            let mut last_half_edge = last_half_edge.write();

            let [_, vertex] = &mut last_half_edge.vertices;
            vertex.1 = shared_surface_vertex;
            last_half_edge.infer_global_form();
        }

        {
            let shared_surface_vertex = {
                let [vertex, _] = &first_half_edge.read().vertices;
                vertex.1.clone()
            };

            let mut new_half_edge = new_half_edge.write();

            let [_, vertex] = &mut new_half_edge.vertices;
            vertex.1 = shared_surface_vertex;
            new_half_edge.replace_surface(self.surface.clone());
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

        {
            let [vertex, _] = &mut half_edge.write().vertices;
            vertex.1.write().position = Some(point.into());
        }

        half_edge
    }

    fn add_half_edge_from_global_point_to_start(
        &mut self,
        point: impl Into<Point<3>>,
    ) -> Partial<HalfEdge> {
        let mut half_edge = self.add_half_edge();

        {
            let [vertex, _] = &mut half_edge.write().vertices;
            vertex.1.write().global_form.write().position = Some(point.into());
        }

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

    fn update_as_triangle_from_global_points(
        &mut self,
        points_global: [impl Into<Point<3>>; 3],
    ) -> [Partial<HalfEdge>; 3] {
        let points_global = points_global.map(Into::into);

        let (points_surface, _) = self
            .surface
            .write()
            .update_as_plane_from_points(points_global);

        let half_edges = self.update_as_polygon_from_points(points_surface);

        for (mut half_edge, point) in half_edges.clone().zip_ext(points_global)
        {
            let [vertex, _] = &mut half_edge.write().vertices;
            vertex.1.write().global_form.write().position = Some(point);
        }

        half_edges
    }

    fn connect_to_open_edges<O>(
        &mut self,
        edges: O,
    ) -> O::SizePlusOne<Partial<HalfEdge>>
    where
        O: ObjectArgument<Partial<HalfEdge>>,
    {
        // We need to create the additional half-edge last, but at the same time
        // need to provide it to the `map_plus_one` method first. Really no
        // choice but to create them all in one go, as we do here.
        let mut half_edges = VecDeque::new();
        for _ in 0..edges.num_objects() {
            half_edges.push_back(self.add_half_edge());
        }
        let additional_half_edge = self.add_half_edge();

        edges.map_plus_one(additional_half_edge, |other| {
            let mut this = half_edges.pop_front().expect(
                "Pushed correct number of half-edges; should be able to pop",
            );
            this.write().update_from_other_edge(&other);
            this
        })
    }

    fn connect_to_closed_edges<O>(
        &mut self,
        edges: O,
    ) -> O::SameSize<Partial<HalfEdge>>
    where
        O: ObjectArgument<Partial<HalfEdge>>,
    {
        edges.map(|other| {
            let mut this = self.add_half_edge();
            this.write().update_from_other_edge(&other);
            this
        })
    }
}
