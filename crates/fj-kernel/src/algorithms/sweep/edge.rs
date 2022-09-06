use fj_interop::mesh::Color;
use fj_math::{Line, Scalar, Transform, Triangle};

use crate::{
    algorithms::{
        approx::{Approx, Tolerance},
        reverse::Reverse,
        transform::TransformObject,
    },
    objects::{
        Curve, CurveKind, Cycle, Edge, Face, GlobalEdge, Surface, Vertex,
        VerticesOfEdge,
    },
};

use super::{Path, Sweep};

impl Sweep for Edge {
    type Swept = Face;

    fn sweep(
        self,
        path: impl Into<Path>,
        tolerance: impl Into<Tolerance>,
        color: Color,
    ) -> Self::Swept {
        let path = path.into();
        let tolerance = tolerance.into();

        if self.vertices().get().is_some() {
            let face =
                create_non_continuous_side_face(&self, path, tolerance, color);
            return face;
        }

        create_continuous_side_face(self, path, tolerance, color)
    }
}

fn create_non_continuous_side_face(
    edge: &Edge,
    path: Path,
    tolerance: Tolerance,
    color: Color,
) -> Face {
    let edge = if path.is_negative_direction() {
        edge.reverse()
    } else {
        *edge
    };

    let surface = edge.curve().sweep(path, tolerance, color);

    // We can't use the edge we're sweeping from as the bottom edge, as that is
    // not defined in the right surface. Let's create a new bottom edge, by
    // swapping the surface of the original.
    let bottom_edge = {
        let vertices = edge.vertices().get_or_panic();

        let points_curve_and_surface = vertices.map(|vertex| {
            (vertex.position(), [vertex.position().t, Scalar::ZERO])
        });

        let curve = {
            // Please note that creating a line here is correct, even if the
            // global curve is a circle. Projected into the side surface, it is
            // going to be a line either way.
            let kind = CurveKind::Line(Line::from_points_with_line_coords(
                points_curve_and_surface,
            ));

            Curve::new(surface, kind, *edge.curve().global_form())
        };

        let vertices = {
            let vertices = vertices.map(|vertex| {
                Vertex::new(vertex.position(), curve, *vertex.global_form())
            });
            VerticesOfEdge::from_vertices(vertices)
        };

        Edge::new(curve, vertices, *edge.global_form())
    };

    let side_edges = bottom_edge
        .vertices()
        .get_or_panic()
        .map(|&vertex| (vertex, surface).sweep(path, tolerance, color));

    let top_edge = {
        let bottom_vertices = bottom_edge.vertices().get_or_panic();

        let global_vertices = side_edges.map(|edge| {
            let [_, vertex] = edge.vertices().get_or_panic();
            *vertex.global_form()
        });

        let points_curve_and_surface = bottom_vertices.map(|vertex| {
            (vertex.position(), [vertex.position().t, Scalar::ONE])
        });

        let curve = {
            let global =
                bottom_edge.curve().global_form().translate(path.inner());

            // Please note that creating a line here is correct, even if the
            // global curve is a circle. Projected into the side surface, it is
            // going to be a line either way.
            let kind = CurveKind::Line(Line::from_points_with_line_coords(
                points_curve_and_surface,
            ));

            Curve::new(surface, kind, global)
        };

        let global = {
            GlobalEdge::new(
                *curve.global_form(),
                VerticesOfEdge::from_vertices(global_vertices),
            )
        };

        let vertices = {
            // Can be cleaned up, once `zip` is stable:
            // https://doc.rust-lang.org/std/primitive.array.html#method.zip
            let [a_vertex, b_vertex] = bottom_vertices;
            let [a_global, b_global] = global_vertices;
            let vertices = [(a_vertex, a_global), (b_vertex, b_global)];

            vertices.map(|(vertex, vertex_global)| {
                Vertex::new(vertex.position(), curve, vertex_global)
            })
        };

        Edge::new(curve, VerticesOfEdge::from_vertices(vertices), global)
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

            let [_, prev_last] = edges[i].vertices().get_or_panic();
            let [next_first, _] = edges[j].vertices().get_or_panic();

            if prev_last.global_form() != next_first.global_form() {
                edges[j] = edges[j].reverse();
            }

            i += 1;
        }

        Cycle::new(surface, edges)
    };

    Face::new(surface).with_exteriors([cycle]).with_color(color)
}

fn create_continuous_side_face(
    edge: Edge,
    path: Path,
    tolerance: Tolerance,
    color: Color,
) -> Face {
    let translation = Transform::translation(path.inner());

    // This is definitely the wrong surface, but it shouldn't matter. Since this
    // code will hopefully soon be gone anyway (this is the last piece of code
    // that prevents us from removing triangle representation), it hopefully
    // won't start to matter at some point either.
    let placeholder = Surface::xy_plane();

    let cycle = Cycle::new(placeholder, [edge]);
    let approx = cycle.approx(tolerance, ());

    let mut quads = Vec::new();
    for segment in approx.segments() {
        let [v0, v1] = segment.points();
        let [v3, v2] = {
            let segment = translation.transform_segment(&segment);
            segment.points()
        };

        quads.push([v0, v1, v2, v3]);
    }

    let mut side_face: Vec<(Triangle<3>, _)> = Vec::new();
    for [v0, v1, v2, v3] in quads {
        side_face.push(([v0, v1, v2].into(), color));
        side_face.push(([v0, v2, v3].into(), color));
    }

    Face::from_triangles(side_face)
}
