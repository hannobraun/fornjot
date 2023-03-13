use fj_interop::{ext::ArrayExt, mesh::Color};
use fj_math::{Point, Scalar, Vector};

use crate::{
    builder::{CycleBuilder, HalfEdgeBuilder},
    insert::Insert,
    objects::{Face, HalfEdge, Objects, Surface, Vertex},
    partial::{PartialFace, PartialObject},
    services::Service,
    storage::Handle,
};

use super::{Sweep, SweepCache};

impl Sweep for (&HalfEdge, &Handle<Vertex>, &Surface, Color) {
    type Swept = (Handle<Face>, Handle<HalfEdge>);

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        objects: &mut Service<Objects>,
    ) -> Self::Swept {
        let (edge, next_vertex, surface, color) = self;
        let path = path.into();

        // The result of sweeping an edge is a face. Let's create that.
        let mut face = PartialFace::new(objects);
        face.color = Some(color);

        // A face (and everything in it) is defined on a surface. A surface can
        // be created by sweeping a curve, so let's sweep the curve of the edge
        // we're sweeping.
        face.surface = Some(
            (edge.curve(), surface).sweep_with_cache(path, cache, objects),
        );

        // Next, we need to define the boundaries of the face. Let's start with
        // the global vertices and edges.
        let (vertices, global_edges) = {
            let [a, b] = [edge.start_vertex(), next_vertex].map(Clone::clone);
            let (edge_up, [_, c]) =
                b.clone().sweep_with_cache(path, cache, objects);
            let (edge_down, [_, d]) =
                a.clone().sweep_with_cache(path, cache, objects);

            (
                [a, b, c, d],
                [
                    Some(edge.global_form().clone()),
                    Some(edge_up),
                    Some(edge_down),
                    None,
                ],
            )
        };

        // Let's figure out the surface coordinates of the edge vertices.
        let surface_points = {
            let [a, b] = edge.boundary();

            [
                [a.t, Scalar::ZERO],
                [b.t, Scalar::ZERO],
                [b.t, Scalar::ONE],
                [a.t, Scalar::ONE],
            ]
            .map(Point::from)
        };
        let surface_points_next = {
            let mut points = surface_points;
            points.rotate_left(1);
            points
        };

        // Now, the boundaries of each edge.
        let boundaries = {
            let [a, b] = edge.boundary();
            let [c, d] = [0., 1.].map(|coord| Point::from([coord]));

            [[a, b], [c, d], [b, a], [d, c]]
        };

        // Armed with all of that, we're ready to create the edges.
        let [_edge_bottom, _edge_up, edge_top, _edge_down] = boundaries
            .zip_ext(surface_points)
            .zip_ext(surface_points_next)
            .zip_ext(vertices)
            .zip_ext(global_edges)
            .map(|((((boundary, start), end), start_vertex), global_edge)| {
                let half_edge = {
                    let builder = HalfEdgeBuilder::line_segment(
                        [start, end],
                        Some(boundary),
                    )
                    .with_start_vertex(start_vertex);

                    let builder = if let Some(global_edge) = global_edge {
                        builder.with_global_form(global_edge)
                    } else {
                        builder
                    };

                    builder.build(objects)
                };

                let (exterior, half_edge) = face
                    .exterior
                    .read()
                    .clone()
                    .add_half_edge(half_edge, objects);
                *face.exterior.write() = exterior;

                half_edge
            });

        // And we're done creating the face! All that's left to do is build our
        // return values.
        let face = face.build(objects).insert(objects);
        (face, edge_top)
    }
}
