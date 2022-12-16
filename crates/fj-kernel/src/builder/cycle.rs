use fj_math::Point;

use crate::{
    objects::{HalfEdge, Surface},
    partial::{Partial, PartialCycle},
};

use super::HalfEdgeBuilder;

/// Builder API for [`PartialCycle`]
pub trait CycleBuilder {
    /// Create a cycle as a polygonal chain from the provided points
    fn update_as_polygon(
        &mut self,
        surface: impl Into<Partial<Surface>>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Vec<Partial<HalfEdge>>;

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
}

impl CycleBuilder for PartialCycle {
    fn update_as_polygon(
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

        for half_edge in &mut self.half_edges {
            half_edge.write().update_as_line_segment();
        }

        half_edges
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
            let shared_surface =
                first_half_edge.read().curve().read().surface.clone();

            let mut new_half_edge = new_half_edge.write();

            new_half_edge.curve().write().surface = shared_surface;
            new_half_edge.front_mut().write().surface_form =
                shared_surface_vertex;
            new_half_edge.infer_global_form();
        }

        self.half_edges.push(new_half_edge.clone());
        new_half_edge
    }
}
