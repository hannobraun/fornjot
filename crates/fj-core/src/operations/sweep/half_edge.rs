use fj_interop::{ext::ArrayExt, mesh::Color};
use fj_math::{Point, Scalar, Vector};

use crate::{
    objects::{Cycle, Face, HalfEdge, Region, Surface, Vertex},
    operations::{
        build::{BuildCycle, BuildHalfEdge},
        insert::Insert,
        update::{UpdateCycle, UpdateHalfEdge},
    },
    services::Services,
    storage::Handle,
};

use super::{Sweep, SweepCache};

impl Sweep for (&HalfEdge, Handle<Vertex>, &Surface, Option<Color>) {
    type Swept = (Face, Handle<HalfEdge>);

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        services: &mut Services,
    ) -> Self::Swept {
        let (edge, next_vertex, surface, color) = self;
        let path = path.into();

        let surface = (edge.path(), surface)
            .sweep_with_cache(path, cache, services)
            .insert(services);

        // Next, we need to define the boundaries of the face. Let's start with
        // the global vertices and edges.
        let (vertices, curves) = {
            let [a, b] = [edge.start_vertex().clone(), next_vertex];
            let (curve_up, c) =
                b.clone().sweep_with_cache(path, cache, services);
            let (curve_down, d) =
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

        let mut exterior = Cycle::empty();

        // Armed with all of that, we're ready to create the edges.
        let [_edge_bottom, _edge_up, edge_top, _edge_down] = boundaries
            .zip_ext(surface_points)
            .zip_ext(surface_points_next)
            .zip_ext(vertices)
            .zip_ext(curves)
            .map(|((((boundary, start), end), start_vertex), curve)| {
                let edge = {
                    let edge = HalfEdge::line_segment(
                        [start, end],
                        Some(boundary),
                        services,
                    )
                    .update_start_vertex(|_| start_vertex);

                    let edge = if let Some(curve) = curve {
                        edge.update_curve(|_| curve)
                    } else {
                        edge
                    };

                    edge.insert(services)
                };

                exterior = exterior.add_half_edges([edge.clone()]);

                edge
            });

        let region =
            Region::new(exterior.insert(services), [], color).insert(services);
        let face = Face::new(surface, region);

        (face, edge_top)
    }
}
