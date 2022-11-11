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
    partial::HasPartial,
    storage::Handle,
    validate::ValidationError,
};

use super::{Sweep, SweepCache};

impl Sweep for (Handle<HalfEdge>, Color) {
    type Swept = Handle<Face>;

    fn sweep_with_cache(
        self,
        path: impl Into<Vector<3>>,
        cache: &mut SweepCache,
        objects: &Objects,
    ) -> Result<Self::Swept, ValidationError> {
        let (edge, color) = self;
        let path = path.into();

        let surface = edge
            .curve()
            .clone()
            .sweep_with_cache(path, cache, objects)?;

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

                objects.curves.insert(Curve::new(
                    surface.clone(),
                    path,
                    edge.curve().global_form().clone(),
                ))?
            };

            let vertices = {
                let points_surface = points_curve_and_surface
                    .map(|(_, point_surface)| point_surface);

                vertices
                    .each_ref_ext()
                    .into_iter_fixed()
                    .zip(points_surface)
                    .collect::<[_; 2]>()
                    .try_map_ext(
                    |(vertex, point_surface)| -> Result<_, ValidationError> {
                        let surface_vertex = objects.surface_vertices.insert(
                            SurfaceVertex::new(
                                point_surface,
                                surface.clone(),
                                vertex.global_form().clone(),
                            ),
                        )?;

                        Ok(objects.vertices.insert(Vertex::new(
                            vertex.position(),
                            curve.clone(),
                            surface_vertex,
                        ))?)
                    },
                )?
            };

            objects
                .half_edges
                .insert(HalfEdge::new(vertices, edge.global_form().clone()))?
        };

        let side_edges =
            bottom_edge.vertices().clone().try_map_ext(|vertex| {
                (vertex, surface.clone()).sweep_with_cache(path, cache, objects)
            })?;

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
                    .translate(path, objects)?;

                // Please note that creating a line here is correct, even if the
                // global curve is a circle. Projected into the side surface, it
                // is going to be a line either way.
                let path =
                    SurfacePath::Line(Line::from_points_with_line_coords(
                        points_curve_and_surface,
                    ));

                objects.curves.insert(Curve::new(surface, path, global))?
            };

            let global = objects.global_edges.insert(GlobalEdge::new(
                curve.global_form().clone(),
                surface_vertices
                    .clone()
                    .map(|surface_vertex| surface_vertex.global_form().clone()),
            ))?;

            let vertices = bottom_vertices
                .each_ref_ext()
                .into_iter_fixed()
                .zip(surface_vertices)
                .collect::<[_; 2]>()
                .try_map_ext(|(vertex, surface_form)| {
                    objects.vertices.insert(Vertex::new(
                        vertex.position(),
                        curve.clone(),
                        surface_form,
                    ))
                })?;

            objects.half_edges.insert(HalfEdge::new(vertices, global))?
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
                    edges[j] = edges[j].clone().reverse(objects)?;
                }

                i += 1;
            }

            objects.cycles.insert(Cycle::new(edges))?
        };

        Ok(Face::partial()
            .with_exterior(cycle)
            .with_color(color)
            .build(objects)?
            .insert(objects)?)
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
        objects::{Cycle, Face, HalfEdge, Objects},
        partial::{HasPartial, PartialSurfaceVertex, PartialVertex},
    };

    #[test]
    fn sweep() -> anyhow::Result<()> {
        let objects = Objects::new();

        let half_edge = HalfEdge::partial()
            .update_as_line_segment_from_points(
                objects.surfaces.xy_plane(),
                [[0., 0.], [1., 0.]],
            )
            .build(&objects)?
            .insert(&objects)?;

        let face =
            (half_edge, Color::default()).sweep([0., 0., 1.], &objects)?;

        let expected_face = {
            let surface = objects.surfaces.xz_plane();

            let bottom = HalfEdge::partial()
                .update_as_line_segment_from_points(
                    surface.clone(),
                    [[0., 0.], [1., 0.]],
                )
                .build(&objects)?
                .insert(&objects)?;
            let side_up = HalfEdge::partial()
                .with_surface(surface.clone())
                .with_back_vertex(PartialVertex {
                    surface_form: bottom.front().surface_form().clone().into(),
                    ..Default::default()
                })
                .with_front_vertex(PartialVertex {
                    surface_form: PartialSurfaceVertex {
                        position: Some([1., 1.].into()),
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                })
                .update_as_line_segment()
                .build(&objects)?
                .insert(&objects)?;
            let top = HalfEdge::partial()
                .with_surface(surface.clone())
                .with_back_vertex(PartialVertex {
                    surface_form: PartialSurfaceVertex {
                        position: Some([0., 1.].into()),
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                })
                .with_front_vertex(PartialVertex {
                    surface_form: side_up.front().surface_form().clone().into(),
                    ..Default::default()
                })
                .update_as_line_segment()
                .build(&objects)?
                .insert(&objects)?
                .reverse(&objects)?;
            let side_down = HalfEdge::partial()
                .with_surface(surface)
                .with_back_vertex(PartialVertex {
                    surface_form: bottom.back().surface_form().clone().into(),
                    ..Default::default()
                })
                .with_front_vertex(PartialVertex {
                    surface_form: top.front().surface_form().clone().into(),
                    ..Default::default()
                })
                .update_as_line_segment()
                .build(&objects)?
                .insert(&objects)?
                .reverse(&objects)?;

            let cycle = objects
                .cycles
                .insert(Cycle::new([bottom, side_up, top, side_down]))?;

            Face::partial()
                .with_exterior(cycle)
                .build(&objects)?
                .insert(&objects)?
        };

        assert_eq!(face, expected_face);
        Ok(())
    }
}
