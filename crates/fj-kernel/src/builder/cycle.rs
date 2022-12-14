use fj_interop::ext::ArrayExt;
use fj_math::Point;

use crate::{
    objects::{Surface, SurfaceVertex},
    partial::{Partial, PartialCycle, PartialHalfEdge, PartialSurfaceVertex},
};

use super::HalfEdgeBuilder;

/// Builder API for [`PartialCycle`]
pub trait CycleBuilder {
    /// Create a cycle as a polygonal chain from the provided points
    fn from_poly_chain(
        surface: impl Into<Partial<Surface>>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self;
}

impl CycleBuilder for PartialCycle {
    fn from_poly_chain(
        surface: impl Into<Partial<Surface>>,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        let surface = surface.into();

        let mut cycle = PartialCycle::default();

        let vertices =
            points.into_iter().map(|position| PartialSurfaceVertex {
                position: Some(position.into()),
                surface: surface.clone(),
                ..Default::default()
            });

        let mut previous: Option<Partial<SurfaceVertex>> =
            cycle.half_edges.last().map(|half_edge| {
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

        cycle.half_edges.extend(half_edges);

        let first = cycle.half_edges.first();
        let last = cycle.half_edges.last();

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
            return cycle;
        };

        let mut half_edge = PartialHalfEdge::default();
        half_edge.curve().write().surface =
            cycle.surface().expect("Need surface to close cycle");

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

        cycle.half_edges.push(Partial::from_partial(half_edge));

        cycle
    }
}
