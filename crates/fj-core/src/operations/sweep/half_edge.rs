use fj_interop::{ext::ArrayExt, Color};
use fj_math::{Point, Scalar, Vector};

use crate::{
    objects::{Cycle, Face, HalfEdge, Region, Surface, Vertex},
    operations::{
        build::{BuildCycle, BuildHalfEdge},
        insert::Insert,
        update::{UpdateCycle, UpdateHalfEdge},
    },
    storage::Handle,
    Instance,
};

use super::{vertex::SweepVertex, SweepCache, SweepSurfacePath};

/// # Sweep a [`HalfEdge`]
///
/// See [module documentation] for more information.
///
/// [module documentation]: super
pub trait SweepHalfEdge {
    /// # Sweep the [`HalfEdge`]
    ///
    /// Returns a face, the result of sweeping the edge, as well as the top edge
    /// of that face, i.e. the edge that is the version of the original edge
    /// that was translated along the sweep path.
    ///
    /// In addition to the usual arguments that many sweep operations require,
    /// some other ones are needed:
    ///
    /// - `end_vertex`, the vertex where the half-edge ends. This is the start
    ///   vertex of the next half-edge in the cycle.
    /// - The `surface` that the half-edge is defined on.
    /// - The `color` of the resulting face, if applicable
    fn sweep_half_edge(
        &self,
        end_vertex: Handle<Vertex>,
        surface: &Surface,
        color: Option<Color>,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        core: &mut Instance,
    ) -> (Face, Handle<HalfEdge>);
}

impl SweepHalfEdge for HalfEdge {
    fn sweep_half_edge(
        &self,
        end_vertex: Handle<Vertex>,
        surface: &Surface,
        color: Option<Color>,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        core: &mut Instance,
    ) -> (Face, Handle<HalfEdge>) {
        let path = path.into();

        let surface = self
            .path()
            .sweep_surface_path(surface, path)
            .insert(&mut core.services);

        // Next, we need to define the boundaries of the face. Let's start with
        // the global vertices and edges.
        let (vertices, curves) = {
            let [a, b] = [self.start_vertex().clone(), end_vertex];
            let (curve_up, c) =
                b.clone().sweep_vertex(cache, &mut core.services);
            let (curve_down, d) =
                a.clone().sweep_vertex(cache, &mut core.services);

            (
                [a, b, c, d],
                [
                    Some(self.curve().clone()),
                    Some(curve_up),
                    None,
                    Some(curve_down),
                ],
            )
        };

        // Let's figure out the surface coordinates of the edge vertices.
        let surface_points = {
            let [a, b] = self.boundary().inner;

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
            let [a, b] = self.boundary().inner;
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
                        &mut core.services,
                    )
                    .update_start_vertex(|_| start_vertex);

                    let edge = if let Some(curve) = curve {
                        edge.update_curve(|_| curve)
                    } else {
                        edge
                    };

                    edge.insert(&mut core.services)
                };

                exterior = exterior.add_half_edges([edge.clone()]);

                edge
            });

        let exterior = exterior.insert(&mut core.services);
        let region =
            Region::new(exterior, [], color).insert(&mut core.services);
        let face = Face::new(surface, region);

        (face, edge_top)
    }
}
