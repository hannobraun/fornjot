use fj_interop::{ext::ArrayExt, mesh::Color};
use fj_math::{Point, Scalar, Vector};

use crate::{
    builder::{CycleBuilder, HalfEdgeBuilder},
    insert::Insert,
    objects::{Face, HalfEdge, Objects, Surface},
    partial::{Partial, PartialFace, PartialObject},
    services::Service,
    storage::Handle,
};

use super::{Sweep, SweepCache};

impl Sweep for (Handle<HalfEdge>, &Surface, Color) {
    type Swept = (Handle<Face>, Handle<HalfEdge>);

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        objects: &mut Service<Objects>,
    ) -> Self::Swept {
        let (edge, surface, color) = self;
        let path = path.into();

        // The result of sweeping an edge is a face. Let's create that.
        let mut face = PartialFace {
            color: Some(color),
            ..Default::default()
        };

        // A face (and everything in it) is defined on a surface. A surface can
        // be created by sweeping a curve, so let's sweep the curve of the edge
        // we're sweeping.
        {
            let surface = Partial::from(
                (edge.curve(), surface).sweep_with_cache(path, cache, objects),
            );
            face.surface = surface;
        }

        // Now we're ready to create the edges.
        let mut edge_bottom = face.exterior.write().add_half_edge();
        let mut edge_up = face.exterior.write().add_half_edge();
        let mut edge_top = face.exterior.write().add_half_edge();
        let mut edge_down = face.exterior.write().add_half_edge();

        // Those edges aren't fully defined yet. We'll do that shortly, but
        // first we have to figure a few things out.
        //
        // Let's start with the global vertices and edges.
        let (global_vertices, global_edges) = {
            let [a, b] = edge
                .surface_vertices()
                .map(|surface_vertex| surface_vertex.global_form().clone());
            let (edge_right, [_, c]) =
                b.clone().sweep_with_cache(path, cache, objects);
            let (edge_left, [_, d]) =
                a.clone().sweep_with_cache(path, cache, objects);

            (
                [a, b, c, d],
                [edge.global_form().clone(), edge_right, edge_left],
            )
        };

        // Next, let's figure out the surface coordinates of the edge vertices.
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

        // Now, the boundaries of each edge.
        let boundaries = {
            let [a, b] = edge.boundary();
            let [c, d] = [0., 1.].map(|coord| Point::from([coord]));

            [[a, b], [c, d], [b, a], [d, c]]
        };

        // Armed with all of that, we can set the edge's vertices.
        [
            edge_bottom.write(),
            edge_up.write(),
            edge_top.write(),
            edge_down.write(),
        ]
        .zip_ext(boundaries)
        .zip_ext(surface_points)
        .zip_ext(global_vertices)
        .map(
            |(((mut half_edge, boundary), surface_point), global_vertex)| {
                for (a, b) in
                    half_edge.boundary.each_mut_ext().zip_ext(boundary)
                {
                    *a = Some(b);
                }

                // Writing to the start vertices is enough. Neighboring half-
                // edges share surface vertices, so writing the start vertex of
                // each half-edge writes to all vertices.
                let mut vertex = half_edge.surface_vertices[0].write();
                vertex.position = Some(surface_point);
                vertex.global_form = Partial::from(global_vertex);
            },
        );

        // With the vertices set, we can now update the curves.
        //
        // Those are all line segments. For the bottom and top curve, because
        // even if the original edge was a circle, it's still going to be a line
        // when projected into the new surface. For the side edges, because
        // we're sweeping along a straight path.
        for mut edge in [
            edge_bottom.write(),
            edge_up.write(),
            edge_top.write(),
            edge_down.write(),
        ] {
            edge.update_as_line_segment();
        }

        // Finally, we can make sure that all edges refer to the correct global
        // edges.
        [edge_bottom.write(), edge_up.write(), edge_down.write()]
            .zip_ext(global_edges)
            .map(|(mut half_edge, global_edge)| {
                half_edge.global_form = Partial::from(global_edge);
            });

        // And we're done creating the face! All that's left to do is build our
        // return values.
        let face = face.build(objects).insert(objects);
        let edge_top = edge_top.build(objects);
        (face, edge_top)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use fj_interop::{ext::ArrayExt, mesh::Color};
    use fj_math::Point;
    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::sweep::Sweep,
        builder::HalfEdgeBuilder,
        insert::Insert,
        partial::{
            Partial, PartialCycle, PartialFace, PartialHalfEdge, PartialObject,
        },
        services::Services,
    };

    #[test]
    fn sweep() {
        let mut services = Services::new();

        let surface = services.objects.surfaces.xy_plane();

        let half_edge = {
            let mut half_edge = PartialHalfEdge::default();
            half_edge.update_as_line_segment_from_points([[0., 0.], [1., 0.]]);
            half_edge.infer_vertex_positions_if_necessary(&surface.geometry());

            half_edge
                .build(&mut services.objects)
                .insert(&mut services.objects)
        };

        let (face, _) = (half_edge, surface.deref(), Color::default())
            .sweep([0., 0., 1.], &mut services.objects);

        let expected_face = {
            let surface = services.objects.surfaces.xz_plane();

            let bottom = {
                let mut half_edge = PartialHalfEdge::default();
                half_edge
                    .update_as_line_segment_from_points([[0., 0.], [1., 0.]]);

                half_edge
            };
            let side_up = {
                let mut side_up = PartialHalfEdge::default();

                {
                    let [back, front] = side_up.surface_vertices.each_mut_ext();

                    *back = bottom.surface_vertices[1].clone();

                    let mut front = front.write();
                    front.position = Some([1., 1.].into());
                }

                side_up.infer_global_form();
                side_up.update_as_line_segment();

                side_up
            };
            let top = {
                let mut top = PartialHalfEdge::default();

                {
                    let [back, front] = top.boundary.each_mut_ext();
                    let [back_surface, front_surface] =
                        top.surface_vertices.each_mut_ext();

                    *back = Some(Point::from([1.]));
                    *back_surface = side_up.surface_vertices[1].clone();

                    *front = Some(Point::from([0.]));
                    let mut front_surface = front_surface.write();
                    front_surface.position = Some([0., 1.].into());
                }

                top.infer_global_form();
                top.update_as_line_segment();
                top.infer_vertex_positions_if_necessary(&surface.geometry());

                Partial::from(
                    top.build(&mut services.objects)
                        .insert(&mut services.objects),
                )
                .read()
                .clone()
            };
            let side_down = {
                let mut side_down = PartialHalfEdge::default();

                let [back, front] = side_down.boundary.each_mut_ext();
                let [back_surface, front_surface] =
                    side_down.surface_vertices.each_mut_ext();

                *back = Some(Point::from([1.]));
                *front = Some(Point::from([0.]));

                *back_surface = top.surface_vertices[1].clone();
                *front_surface = bottom.surface_vertices[0].clone();

                side_down.infer_global_form();
                side_down.update_as_line_segment();
                side_down
                    .infer_vertex_positions_if_necessary(&surface.geometry());

                Partial::from(
                    side_down
                        .build(&mut services.objects)
                        .insert(&mut services.objects),
                )
                .read()
                .clone()
            };

            let mut cycle = PartialCycle::default();
            cycle.half_edges.extend(
                [bottom, side_up, top, side_down].map(Partial::from_partial),
            );

            let face = PartialFace {
                surface: Partial::from(surface),
                exterior: Partial::from_partial(cycle),
                ..Default::default()
            };
            face.build(&mut services.objects)
                .insert(&mut services.objects)
        };

        assert_eq!(face, expected_face);
    }
}
