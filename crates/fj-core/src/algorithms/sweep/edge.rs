use fj_interop::{ext::ArrayExt, mesh::Color};
use fj_math::{Point, Scalar, Vector};

use crate::{
    objects::{Cycle, Edge, Face, Region, Surface, Vertex},
    operations::{BuildEdge, Insert, UpdateCycle, UpdateHalfEdge},
    services::Services,
    storage::Handle,
};

use super::{Sweep, SweepCache};

impl Sweep for (&Edge, &Handle<Vertex>, &Surface, Option<Color>) {
    type Swept = (Handle<Face>, Handle<Edge>);

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        services: &mut Services,
    ) -> Self::Swept {
        let (edge, next_vertex, surface, color) = self;
        let path = path.into();

        let surface =
            (edge.path(), surface).sweep_with_cache(path, cache, services);

        // Next, we need to define the boundaries of the face. Let's start with
        // the global vertices and edges.
        let (vertices, curves) = {
            let [a, b] = [edge.start_vertex(), next_vertex].map(Clone::clone);
            let (curve_up, [_, c]) =
                b.clone().sweep_with_cache(path, cache, services);
            let (curve_down, [_, d]) =
                a.clone().sweep_with_cache(path, cache, services);

            (
                [a, b, c, d],
                [
                    Some(edge.curve().clone()),
                    Some(curve_up),
                    None,
                    Some(curve_down),
                ],
            )
        };

        // Let's figure out the surface coordinates of the edge vertices.
        let surface_points = {
            let [a, b] = edge.boundary().inner;

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
            let [a, b] = edge.boundary().inner;
            let [c, d] = [0., 1.].map(|coord| Point::from([coord]));

            [[a, b], [c, d], [b, a], [d, c]]
        };

        let mut exterior = Some(Cycle::new([]));

        // Armed with all of that, we're ready to create the edges.
        let [_edge_bottom, _edge_up, edge_top, _edge_down] = boundaries
            .zip_ext(surface_points)
            .zip_ext(surface_points_next)
            .zip_ext(vertices)
            .zip_ext(curves)
            .map(|((((boundary, start), end), start_vertex), curve)| {
                let half_edge = {
                    let half_edge = Edge::line_segment(
                        [start, end],
                        Some(boundary),
                        services,
                    )
                    .replace_start_vertex(start_vertex);

                    let half_edge = if let Some(curve) = curve {
                        half_edge.replace_curve(curve)
                    } else {
                        half_edge
                    };

                    half_edge.insert(services)
                };

                exterior = Some(
                    exterior
                        .take()
                        .unwrap()
                        .add_half_edges([half_edge.clone()]),
                );

                half_edge
            });

        let region = Region::new(exterior.unwrap().insert(services), [], color)
            .insert(services);

        let face = Face::new(surface, region);

        // And we're done creating the face! All that's left to do is build our
        // return values.
        let face = face.insert(services);
        (face, edge_top)
    }
}
