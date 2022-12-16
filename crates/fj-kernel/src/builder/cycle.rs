use fj_math::Point;

use crate::{
    objects::{HalfEdge, Surface},
    partial::{Partial, PartialCycle},
};

use super::HalfEdgeBuilder;

/// Builder API for [`PartialCycle`]
pub trait CycleBuilder {
    /// Create a cycle as a polygonal chain from the provided points
    fn update_as_polygon_from_points(
        &mut self,
        surface: impl Into<Partial<Surface>>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Vec<Partial<HalfEdge>>;

    /// Update cycle to be a polygon
    ///
    /// Will update each half-edge in the cycle to be a line segment.
    fn update_as_polygon(&mut self);

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
}

impl CycleBuilder for PartialCycle {
    fn update_as_polygon_from_points(
        &mut self,
        surface: impl Into<Partial<Surface>>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Vec<Partial<HalfEdge>> {
        let surface = surface.into();

        let mut half_edges = Vec::new();

        for point in points.into_iter().map(Into::into) {
            let mut half_edge = self.add_half_edge();

            {
                let mut half_edge = half_edge.write();

                half_edge.curve().write().surface = surface.clone();

                let mut back = half_edge.back_mut().write();
                let mut back_surface = back.surface_form.write();

                back_surface.position = Some(point);
                back_surface.surface = surface.clone();
            }

            half_edges.push(half_edge);
        }

        self.update_as_polygon();

        half_edges
    }

    fn update_as_polygon(&mut self) {
        for half_edge in &mut self.half_edges {
            half_edge.write().update_as_line_segment();
        }
    }

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
                new_half_edge.read().back().read().surface_form.clone();

            let mut last_half_edge = last_half_edge.write();

            last_half_edge.front_mut().write().surface_form =
                shared_surface_vertex;
            last_half_edge.infer_global_form();
        }

        {
            let shared_surface_vertex =
                first_half_edge.read().back().read().surface_form.clone();
            let shared_surface = shared_surface_vertex.read().surface.clone();

            let mut new_half_edge = new_half_edge.write();

            new_half_edge.front_mut().write().surface_form =
                shared_surface_vertex;
            new_half_edge.replace_surface(shared_surface);
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

        half_edge
            .write()
            .back_mut()
            .write()
            .surface_form
            .write()
            .position = Some(point.into());

        half_edge
    }
}
