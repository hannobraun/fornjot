use fj_interop::ext::ArrayExt;
use fj_math::Point;

use crate::{
    objects::{Surface, SurfaceVertex},
    partial::{Partial, PartialCycle, PartialHalfEdge, PartialSurfaceVertex},
    storage::Handle,
};

use super::HalfEdgeBuilder;

/// Builder API for [`PartialCycle`]
pub trait CycleBuilder {
    /// Update the partial cycle with a polygonal chain from the provided points
    fn with_poly_chain(
        self,
        vertices: impl IntoIterator<Item = PartialSurfaceVertex>,
    ) -> Self;

    /// Update the partial cycle with a polygonal chain from the provided points
    fn with_poly_chain_from_points(
        self,
        surface: Handle<Surface>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self;

    /// Update the partial cycle by closing it with a line segment
    ///
    /// Builds a line segment from the last and first vertex, closing the cycle.
    fn close_with_line_segment(self) -> Self;
}

impl CycleBuilder for PartialCycle {
    fn with_poly_chain(
        mut self,
        vertices: impl IntoIterator<Item = PartialSurfaceVertex>,
    ) -> Self {
        let vertices = vertices.into_iter();

        let mut previous: Option<Partial<SurfaceVertex>> =
            self.half_edges.last().map(|half_edge| {
                let [_, last] = &half_edge.read().vertices;
                let last = last.read();
                last.surface_form.clone()
            });

        let mut half_edges = Vec::new();
        for vertex_next in vertices {
            let vertex_next = Partial::from_partial(vertex_next);

            if let Some(vertex_prev) = previous {
                let surface = vertex_prev.read().surface.clone();

                previous = Some(vertex_next.clone());
                let surface_vertices = [vertex_prev, vertex_next];

                let mut half_edge = PartialHalfEdge::default();
                half_edge.curve().write().surface = surface;

                {
                    let global_vertices =
                        &mut half_edge.global_form.write().vertices;

                    for ((vertex, surface_form), global_form) in half_edge
                        .vertices
                        .each_mut_ext()
                        .zip_ext(surface_vertices)
                        .zip_ext(global_vertices.each_mut_ext())
                    {
                        *global_form = surface_form.read().global_form.clone();
                        vertex.write().surface_form = surface_form;
                    }
                }

                half_edge.update_as_line_segment();
                half_edges.push(Partial::from_partial(half_edge));

                continue;
            }

            previous = Some(vertex_next);
        }

        self.half_edges.extend(half_edges);
        self
    }

    fn with_poly_chain_from_points(
        self,
        surface: Handle<Surface>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        self.with_poly_chain(points.into_iter().map(|position| {
            PartialSurfaceVertex {
                position: Some(position.into()),
                surface: Partial::from_full_entry_point(surface.clone()),
                ..Default::default()
            }
        }))
    }

    fn close_with_line_segment(mut self) -> Self {
        let first = self.half_edges.first();
        let last = self.half_edges.last();

        let vertices = [first, last].map(|option| {
            option.map(|half_edge| {
                half_edge
                    .read()
                    .vertices
                    .each_ref_ext()
                    .map(|vertex| vertex.read().surface_form.clone())
            })
        });

        let [Some([first, _]), Some([_, last])] = vertices else {
            return self;
        };

        let mut half_edge = PartialHalfEdge::default();
        half_edge.curve().write().surface =
            self.surface().expect("Need surface to close cycle");

        {
            let global_vertices = &mut half_edge.global_form.write().vertices;

            for ((vertex, surface_form), global_form) in half_edge
                .vertices
                .each_mut_ext()
                .zip_ext([last, first])
                .zip_ext(global_vertices.each_mut_ext())
            {
                *global_form = surface_form.read().global_form.clone();
                vertex.write().surface_form = surface_form;
            }
        }

        half_edge.update_as_line_segment();

        self.half_edges.push(Partial::from_partial(half_edge));
        self
    }
}
