use fj_interop::mesh::Color;
use fj_math::{Point, Transform, Triangle};

use crate::{
    algorithms::{
        approx::{Approx, Tolerance},
        reverse::Reverse,
    },
    objects::{
        Curve, CurveKind, Cycle, Edge, Face, GlobalCurve, Surface, Vertex,
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
    let vertices_bottom = edge.vertices().get_or_panic();

    let vertices = {
        let vertices_bottom = vertices_bottom.map(|vertex| *vertex.global());

        let vertices_top = vertices_bottom.map(|vertex| {
            let side_edge = vertex.sweep(path, tolerance, color);
            let [_, &vertex_top] = side_edge.vertices().get_or_panic();
            vertex_top
        });

        let [[a, b], [c, d]] = [vertices_bottom, vertices_top];

        if path.is_negative_direction() {
            [b, a, c, d]
        } else {
            [a, b, d, c]
        }
    };

    let surface = {
        let edge = if path.is_negative_direction() {
            edge.reverse()
        } else {
            *edge
        };

        edge.curve().sweep(path, tolerance, color)
    };

    let cycle = {
        let [a, b, c, d] = vertices;

        let mut vertices =
            vec![([0., 0.], a), ([1., 0.], b), ([1., 1.], c), ([0., 1.], d)];
        if let Some(vertex) = vertices.first().cloned() {
            vertices.push(vertex);
        }

        let mut edges = Vec::new();
        for vertices in vertices.windows(2) {
            // Can't panic, as we passed `2` to `windows`.
            //
            // Can be cleaned up, once `array_windows` is stable"
            // https://doc.rust-lang.org/std/primitive.slice.html#method.array_windows
            let [a, b] = [&vertices[0], &vertices[1]];

            let curve = {
                let local = CurveKind::line_from_points([a.0, b.0]);

                let global = [a, b].map(|vertex| vertex.1.position());
                let global =
                    GlobalCurve::from_kind(CurveKind::line_from_points(global));

                Curve::new(surface, local, global)
            };

            let vertices = VerticesOfEdge::from_vertices([
                Vertex::new(Point::from([0.]), curve, a.1),
                Vertex::new(Point::from([1.]), curve, b.1),
            ]);

            let edge = Edge::from_curve_and_vertices(curve, vertices);

            edges.push(edge);
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
