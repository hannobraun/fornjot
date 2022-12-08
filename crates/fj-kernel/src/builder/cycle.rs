use fj_interop::ext::ArrayExt;
use fj_math::Point;

use crate::{
    objects::{Curve, Surface, SurfaceVertex, Vertex},
    partial::{PartialCycle, PartialHalfEdge},
    partial2::{
        Partial, PartialCurve, PartialGlobalEdge, PartialSurfaceVertex,
        PartialVertex,
    },
    storage::Handle,
};

use super::{CurveBuilder, HalfEdgeBuilder};

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
        self,
        vertices: impl IntoIterator<Item = PartialSurfaceVertex>,
    ) -> Self {
        let vertices = vertices.into_iter();

        let mut previous: Option<Partial<SurfaceVertex>> =
            self.half_edges().last().map(|half_edge| {
                let [_, last] = half_edge.vertices();
                let last = last.read();
                last.surface_form.clone()
            });

        let mut half_edges = Vec::new();
        for vertex_next in vertices {
            let vertex_next = Partial::from_partial(vertex_next);

            if let Some(vertex_prev) = previous {
                let surface = vertex_prev.read().surface.clone();

                let [position_prev, position_next] =
                    [&vertex_prev, &vertex_next].map(|vertex| {
                        vertex
                            .read()
                            .position
                            .expect("Need surface position to extend cycle")
                    });

                previous = Some(vertex_next.clone());

                let mut curve: Partial<Curve> =
                    Partial::from_partial(PartialCurve {
                        surface: surface.clone(),
                        ..Default::default()
                    });
                curve
                    .write()
                    .update_as_line_from_points([position_prev, position_next]);

                let vertices = [(0., vertex_prev), (1., vertex_next)].map(
                    |(position, surface_form)| {
                        Partial::from_partial(PartialVertex {
                            position: Some([position].into()),
                            curve: curve.clone(),
                            surface_form,
                        })
                    },
                );

                let global_vertices =
                    vertices.each_ref_ext().map(|vertex: &Partial<Vertex>| {
                        vertex.read().surface_form.read().global_form.clone()
                    });

                half_edges.push(PartialHalfEdge {
                    vertices,
                    global_form: Partial::from_partial(PartialGlobalEdge {
                        curve: curve.read().global_form.clone(),
                        vertices: global_vertices,
                    }),
                });

                continue;
            }

            previous = Some(vertex_next);
        }

        self.with_half_edges(half_edges)
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

    fn close_with_line_segment(self) -> Self {
        let first = self.half_edges().next();
        let last = self.half_edges().last();

        let vertices = [first, last]
            .map(|option| option.map(|half_edge| half_edge.vertices()));

        let [Some([first, _]), Some([_, last])] = vertices else {
            return self;
        };

        let surface = self.surface().expect("Need surface to close cycle");
        let vertices = [last, first].map(|vertex| {
            Partial::<Vertex>::from_partial(PartialVertex {
                curve: Partial::from_partial(PartialCurve {
                    surface: surface.clone(),
                    ..Default::default()
                }),
                surface_form: vertex.read().surface_form.clone(),
                ..Default::default()
            })
        });
        let curve = {
            let [vertex, _] = &vertices;
            vertex.read().curve.read().global_form.clone()
        };
        let global_vertices = vertices.each_ref_ext().map(|vertex| {
            vertex.read().surface_form.read().global_form.clone()
        });

        let half_edge = PartialHalfEdge {
            vertices,
            global_form: Partial::from_partial(PartialGlobalEdge {
                curve,
                vertices: global_vertices,
            }),
        }
        .update_as_line_segment();

        self.with_half_edges(Some(half_edge))
    }
}
