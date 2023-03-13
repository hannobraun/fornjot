use fj_math::Point;

use crate::{
    geometry::curve::Curve,
    insert::Insert,
    objects::{HalfEdge, Objects},
    partial::PartialCycle,
    services::Service,
    storage::Handle,
};

use super::{HalfEdgeBuilder, ObjectArgument};

/// Builder API for [`PartialCycle`]
pub trait CycleBuilder: Sized {
    /// Add a new half-edge to the cycle
    ///
    /// Creates a half-edge and adds it to the cycle. The new half-edge is
    /// connected to the front vertex of the last half-edge , and the back
    /// vertex of the first edge, making sure the half-edges actually form a
    /// cycle.
    ///
    /// If this is the first half-edge being added, it is connected to itself,
    /// meaning its front and back vertices are the same.
    fn add_half_edge(
        self,
        half_edge: HalfEdge,
        objects: &mut Service<Objects>,
    ) -> (Self, Handle<HalfEdge>);

    /// Update cycle as a polygon from the provided points
    fn update_as_polygon_from_points<O, P>(
        self,
        points: O,
        objects: &mut Service<Objects>,
    ) -> (Self, O::SameSize<Handle<HalfEdge>>)
    where
        O: ObjectArgument<P>,
        P: Clone + Into<Point<2>>;

    /// Connect the cycles to the provided half-edges
    ///
    /// Assumes that the provided half-edges, once translated into local
    /// equivalents of this cycle, form a cycle themselves.
    ///
    /// Returns the local equivalents of the provided half-edges.
    fn connect_to_edges<O>(
        self,
        edges: O,
        objects: &mut Service<Objects>,
    ) -> (Self, O::SameSize<Handle<HalfEdge>>)
    where
        O: ObjectArgument<(Handle<HalfEdge>, Curve, [Point<1>; 2])>;
}

impl CycleBuilder for PartialCycle {
    fn add_half_edge(
        mut self,
        half_edge: HalfEdge,
        objects: &mut Service<Objects>,
    ) -> (Self, Handle<HalfEdge>) {
        let half_edge = half_edge.insert(objects);
        self.half_edges.push(half_edge.clone());
        (self, half_edge)
    }

    fn update_as_polygon_from_points<O, P>(
        mut self,
        points: O,
        objects: &mut Service<Objects>,
    ) -> (Self, O::SameSize<Handle<HalfEdge>>)
    where
        O: ObjectArgument<P>,
        P: Clone + Into<Point<2>>,
    {
        let half_edges = points.map_with_next(|start, end| {
            let half_edge = HalfEdgeBuilder::line_segment([start, end], None)
                .build(objects);

            let (cycle, half_edge) =
                self.clone().add_half_edge(half_edge, objects);
            self = cycle;

            half_edge
        });

        (self, half_edges)
    }

    fn connect_to_edges<O>(
        mut self,
        edges: O,
        objects: &mut Service<Objects>,
    ) -> (Self, O::SameSize<Handle<HalfEdge>>)
    where
        O: ObjectArgument<(Handle<HalfEdge>, Curve, [Point<1>; 2])>,
    {
        let edges =
            edges.map_with_prev(|(_, curve, boundary), (prev, _, _)| {
                let half_edge = HalfEdgeBuilder::new(curve, boundary)
                    .with_start_vertex(prev.start_vertex().clone())
                    .build(objects);

                let (cycle, half_edge) =
                    self.clone().add_half_edge(half_edge, objects);
                self = cycle;

                half_edge
            });

        (self, edges)
    }
}
