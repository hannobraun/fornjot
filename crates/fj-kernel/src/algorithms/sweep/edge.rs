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
        stores: &Objects,
    ) -> Self::Swept {
        let (edge, color) = self;
        let path = path.into();

        let surface = edge.curve().clone().sweep(path, stores);

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
                    stores,
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
            .map(|vertex| (vertex, surface.clone()).sweep(path, stores));

        let top_edge = {
            let bottom_vertices = bottom_edge.vertices();

            let global_vertices = side_edges.clone().map(|edge| {
                let [_, vertex] = edge.vertices();
                vertex.global_form().clone()
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
                    .translate(path, stores);

                // Please note that creating a line here is correct, even if the
                // global curve is a circle. Projected into the side surface, it
                // is going to be a line either way.
                let path =
                    SurfacePath::Line(Line::from_points_with_line_coords(
                        points_curve_and_surface,
                    ));

                Curve::new(surface.clone(), path, global, stores)
            };

            let global = GlobalEdge::new(
                curve.global_form().clone(),
                global_vertices.clone(),
            );

            let vertices = {
                let surface_points = points_curve_and_surface
                    .map(|(_, point_surface)| point_surface);

                // Can be cleaned up, once `zip` is stable:
                // https://doc.rust-lang.org/std/primitive.array.html#method.zip
                let [a_vertex, b_vertex] = bottom_vertices;
                let [a_surface, b_surface] = surface_points;
                let [a_global, b_global] = global_vertices;
                let vertices = [
                    (a_vertex, a_surface, a_global),
                    (b_vertex, b_surface, b_global),
                ];

                vertices.map(|(vertex, point_surface, global_form)| {
                    let surface_form = SurfaceVertex::new(
                        point_surface,
                        surface.clone(),
                        global_form,
                    );
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
                if prev_last.surface_form() != next_first.surface_form() {
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
        objects::{Cycle, Face, HalfEdge, Objects, Surface},
        partial::HasPartial,
    };

    #[test]
    fn sweep() {
        let stores = Objects::new();

        let half_edge = HalfEdge::partial()
            .with_surface(Some(stores.surfaces.insert(Surface::xy_plane())))
            .as_line_segment_from_points([[0., 0.], [1., 0.]])
            .build(&stores);

        let face = (half_edge, Color::default()).sweep([0., 0., 1.], &stores);

        let expected_face = {
            let surface = stores.surfaces.insert(Surface::xz_plane());

            let bottom = HalfEdge::partial()
                .with_surface(Some(surface.clone()))
                .as_line_segment_from_points([[0., 0.], [1., 0.]])
                .build(&stores);
            let top = HalfEdge::partial()
                .with_surface(Some(surface.clone()))
                .as_line_segment_from_points([[0., 1.], [1., 1.]])
                .build(&stores)
                .reverse();
            let left = HalfEdge::partial()
                .with_surface(Some(surface.clone()))
                .as_line_segment_from_points([[0., 0.], [0., 1.]])
                .build(&stores)
                .reverse();
            let right = HalfEdge::partial()
                .with_surface(Some(surface.clone()))
                .as_line_segment_from_points([[1., 0.], [1., 1.]])
                .build(&stores);

            let cycle = Cycle::new(surface, [bottom, right, top, left]);

            Face::from_exterior(cycle)
        };

        assert_eq!(face, expected_face);
    }
}
