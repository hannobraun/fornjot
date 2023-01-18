use fj_interop::{ext::ArrayExt, mesh::Color};
use fj_math::{Line, Scalar, Vector};
use iter_fixed::IntoIteratorFixed;

use crate::{
    algorithms::{reverse::Reverse, transform::TransformObject},
    geometry::path::SurfacePath,
    insert::Insert,
    objects::{
        Curve, Cycle, Face, GlobalEdge, HalfEdge, Objects, SurfaceVertex,
        Vertex,
    },
    partial::{Partial, PartialFace, PartialObject},
    services::Service,
    storage::Handle,
};

use super::{Sweep, SweepCache};

impl Sweep for (Handle<HalfEdge>, Color) {
    type Swept = Handle<Face>;

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        objects: &mut Service<Objects>,
    ) -> Self::Swept {
        let (edge, color) = self;
        let path = path.into();

        let surface =
            edge.curve().clone().sweep_with_cache(path, cache, objects);

        // We can't use the edge we're sweeping from as the bottom edge, as that
        // is not defined in the right surface. Let's create a new bottom edge,
        // by swapping the surface of the original.
        let bottom_edge = {
            let vertices = edge.vertices();

            let points_curve_and_surface = vertices.clone().map(|vertex| {
                (vertex.position(), [vertex.position().t, Scalar::ZERO])
            });

            let curve = {
                // Please note that creating a line here is correct, even if the
                // global curve is a circle. Projected into the side surface, it
                // is going to be a line either way.
                let path =
                    SurfacePath::Line(Line::from_points_with_line_coords(
                        points_curve_and_surface,
                    ));

                Curve::new(
                    surface.clone(),
                    path,
                    edge.curve().global_form().clone(),
                )
                .insert(objects)
            };

            let vertices = {
                let points_surface = points_curve_and_surface
                    .map(|(_, point_surface)| point_surface);

                vertices
                    .each_ref_ext()
                    .into_iter_fixed()
                    .zip(points_surface)
                    .collect::<[_; 2]>()
                    .map(|(vertex, point_surface)| {
                        let surface_vertex = SurfaceVertex::new(
                            point_surface,
                            surface.clone(),
                            vertex.surface_form().global_form().clone(),
                        )
                        .insert(objects);

                        Vertex::new(
                            vertex.position(),
                            curve.clone(),
                            surface_vertex,
                        )
                        .insert(objects)
                    })
            };

            HalfEdge::new(vertices, edge.global_form().clone()).insert(objects)
        };

        let side_edges = bottom_edge.vertices().clone().map(|vertex| {
            (vertex.clone_object(), surface.clone())
                .sweep_with_cache(path, cache, objects)
        });

        let top_edge = {
            let bottom_vertices = bottom_edge.vertices();

            let surface_vertices = side_edges.clone().map(|edge| {
                let [_, vertex] = edge.vertices();
                vertex.surface_form().clone()
            });

            let points_curve_and_surface =
                bottom_vertices.clone().map(|vertex| {
                    (vertex.position(), [vertex.position().t, Scalar::ONE])
                });

            let curve = {
                let global = bottom_edge
                    .curve()
                    .global_form()
                    .clone()
                    .translate(path, objects);

                // Please note that creating a line here is correct, even if the
                // global curve is a circle. Projected into the side surface, it
                // is going to be a line either way.
                let path =
                    SurfacePath::Line(Line::from_points_with_line_coords(
                        points_curve_and_surface,
                    ));

                Curve::new(surface, path, global).insert(objects)
            };

            let global = GlobalEdge::new(
                curve.global_form().clone(),
                surface_vertices
                    .clone()
                    .map(|surface_vertex| surface_vertex.global_form().clone()),
            )
            .insert(objects);

            let vertices = bottom_vertices
                .each_ref_ext()
                .into_iter_fixed()
                .zip(surface_vertices)
                .collect::<[_; 2]>()
                .map(|(vertex, surface_form)| {
                    Vertex::new(vertex.position(), curve.clone(), surface_form)
                        .insert(objects)
                });

            HalfEdge::new(vertices, global).insert(objects)
        };

        let cycle = {
            let a = bottom_edge;
            let [d, b] = side_edges;
            let c = top_edge;

            let mut edges = [a, b, c, d];

            // Make sure that edges are oriented correctly.
            let mut i = 0;
            while i < edges.len() {
                let j = (i + 1) % edges.len();

                let [_, prev_last] = edges[i].vertices();
                let [next_first, _] = edges[j].vertices();

                // Need to compare surface forms here, as the global forms might
                // be coincident when sweeping circles, despite the vertices
                // being different!
                if prev_last.surface_form().id()
                    != next_first.surface_form().id()
                {
                    edges[j] = edges[j].clone().reverse(objects);
                }

                i += 1;
            }

            Cycle::new(edges).insert(objects)
        };

        let face = PartialFace {
            exterior: Partial::from(cycle),
            color: Some(color),
            ..Default::default()
        };
        face.build(objects).insert(objects)
    }
}

#[cfg(test)]
mod tests {
    use fj_interop::mesh::Color;
    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::{reverse::Reverse, sweep::Sweep},
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

        let half_edge = {
            let mut half_edge = PartialHalfEdge::default();
            half_edge.update_as_line_segment_from_points(
                services.objects.surfaces.xy_plane(),
                [[0., 0.], [1., 0.]],
            );

            half_edge
                .build(&mut services.objects)
                .insert(&mut services.objects)
        };

        let face = (half_edge, Color::default())
            .sweep([0., 0., 1.], &mut services.objects);

        let expected_face = {
            let surface = Partial::from(services.objects.surfaces.xz_plane());

            let bottom = {
                let mut half_edge = PartialHalfEdge::default();
                half_edge.update_as_line_segment_from_points(
                    surface.clone(),
                    [[0., 0.], [1., 0.]],
                );

                half_edge
            };
            let side_up = {
                let mut side_up = PartialHalfEdge::default();
                side_up.curve().write().surface = surface.clone();

                {
                    let [back, front] = &mut side_up.vertices;

                    back.write().surface_form =
                        bottom.vertices[1].read().surface_form.clone();

                    let mut front = front.write();
                    let mut front = front.surface_form.write();
                    front.position = Some([1., 1.].into());
                    front.surface = surface.clone();
                }

                side_up.infer_global_form();
                side_up.update_as_line_segment();

                side_up
            };
            let top = {
                let mut top = PartialHalfEdge::default();
                top.curve().write().surface = surface.clone();

                {
                    let [back, front] = &mut top.vertices;

                    let mut back = back.write();
                    let mut back = back.surface_form.write();
                    back.position = Some([0., 1.].into());
                    back.surface = surface.clone();

                    front.write().surface_form =
                        side_up.vertices[1].read().surface_form.clone();
                }

                top.infer_global_form();
                top.update_as_line_segment();

                Partial::from(
                    top.build(&mut services.objects)
                        .insert(&mut services.objects)
                        .reverse(&mut services.objects),
                )
                .read()
                .clone()
            };
            let side_down = {
                let mut side_down = PartialHalfEdge::default();
                side_down.curve().write().surface = surface;

                let [back, front] = &mut side_down.vertices;

                back.write().surface_form =
                    bottom.vertices[0].read().surface_form.clone();
                front.write().surface_form =
                    top.vertices[1].read().surface_form.clone();

                side_down.infer_global_form();
                side_down.update_as_line_segment();

                Partial::from(
                    side_down
                        .build(&mut services.objects)
                        .insert(&mut services.objects)
                        .reverse(&mut services.objects),
                )
                .read()
                .clone()
            };

            let mut cycle = PartialCycle::default();
            cycle.half_edges.extend(
                [bottom, side_up, top, side_down].map(Partial::from_partial),
            );

            let face = PartialFace {
                exterior: Partial::from_partial(cycle),
                ..Default::default()
            };
            face.build(&mut services.objects)
                .insert(&mut services.objects)
        };

        assert_eq!(face, expected_face);
    }
}
