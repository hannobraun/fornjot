use fj_interop::mesh::Color;
use fj_math::{Line, Scalar, Vector};

use crate::{
    algorithms::{reverse::Reverse, transform::TransformObject},
    objects::{
        Curve, Cycle, Face, GlobalEdge, HalfEdge, Objects, SurfaceVertex,
        Vertex,
    },
    path::SurfacePath,
};

use super::Sweep;

impl Sweep for (HalfEdge, Color) {
    type Swept = Face;

    fn sweep(
        self,
        path: impl Into<Vector<3>>,
        objects: &Objects,
    ) -> Self::Swept {
        let (edge, color) = self;
        let path = path.into();

        let surface = edge.curve().clone().sweep(path, objects);

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
                    objects,
                )
            };

            let vertices = {
                let points_surface = points_curve_and_surface
                    .map(|(_, point_surface)| point_surface);

                // Can be cleaned up, once `zip` is stable:
                // https://doc.rust-lang.org/std/primitive.array.html#method.zip
                let [a_vertex, b_vertex] = vertices;
                let [a_surface, b_surface] = points_surface;
                let vertices_with_surface_points =
                    [(a_vertex, a_surface), (b_vertex, b_surface)];

                vertices_with_surface_points.map(|(vertex, point_surface)| {
                    let surface_vertex = SurfaceVertex::new(
                        point_surface,
                        surface.clone(),
                        vertex.global_form().clone(),
                        objects,
                    );

                    Vertex::new(
                        vertex.position(),
                        curve.clone(),
                        surface_vertex,
                    )
                })
            };

            HalfEdge::new(vertices, edge.global_form().clone())
        };

        let side_edges = bottom_edge
            .vertices()
            .clone()
            .map(|vertex| (vertex, surface.clone()).sweep(path, objects));

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

                Curve::new(surface.clone(), path, global, objects)
            };

            let global = GlobalEdge::new(
                curve.global_form().clone(),
                surface_vertices
                    .clone()
                    .map(|surface_vertex| surface_vertex.global_form().clone()),
            );

            let vertices = {
                // Can be cleaned up, once `zip` is stable:
                // https://doc.rust-lang.org/std/primitive.array.html#method.zip
                let [a_vertex, b_vertex] = bottom_vertices;
                let [a_surface, b_surface] = surface_vertices;
                let vertices = [(a_vertex, a_surface), (b_vertex, b_surface)];

                vertices.map(|(vertex, surface_form)| {
                    Vertex::new(vertex.position(), curve.clone(), surface_form)
                })
            };

            HalfEdge::new(vertices, global)
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
                    edges[j] = edges[j].clone().reverse();
                }

                i += 1;
            }

            Cycle::new(surface, edges)
        };

        Face::from_exterior(cycle).with_color(color)
    }
}

#[cfg(test)]
mod tests {
    use fj_interop::mesh::Color;
    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::{reverse::Reverse, sweep::Sweep},
        objects::{Cycle, Face, HalfEdge, Objects},
        partial::HasPartial,
    };

    #[test]
    fn sweep() {
        let objects = Objects::new();

        let half_edge = HalfEdge::partial()
            .with_surface(Some(objects.surfaces.xy_plane()))
            .as_line_segment_from_points([[0., 0.], [1., 0.]])
            .build(&objects);

        let face = (half_edge, Color::default()).sweep([0., 0., 1.], &objects);

        let expected_face = {
            let surface = objects.surfaces.xz_plane();

            let bottom = HalfEdge::partial()
                .with_surface(Some(surface.clone()))
                .as_line_segment_from_points([[0., 0.], [1., 0.]])
                .build(&objects);
            let top = HalfEdge::partial()
                .with_surface(Some(surface.clone()))
                .as_line_segment_from_points([[0., 1.], [1., 1.]])
                .build(&objects)
                .reverse();
            let side_down = HalfEdge::partial()
                .with_surface(Some(surface.clone()))
                .as_line_segment_from_points([[0., 0.], [0., 1.]])
                .build(&objects)
                .reverse();
            let right = HalfEdge::partial()
                .with_surface(Some(surface.clone()))
                .as_line_segment_from_points([[1., 0.], [1., 1.]])
                .build(&objects);

            let cycle = Cycle::new(surface, [bottom, right, top, side_down]);

            Face::from_exterior(cycle)
        };

        assert_eq!(face, expected_face);
    }
}
