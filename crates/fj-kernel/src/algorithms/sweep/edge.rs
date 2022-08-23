use fj_interop::mesh::Color;
use fj_math::{Point, Scalar, Transform, Triangle, Vector};

use crate::{
    algorithms::{CycleApprox, Tolerance},
    objects::{
        Curve, CurveKind, Cycle, Edge, Face, GlobalCurve, GlobalVertex,
        Surface, Vertex, VerticesOfEdge,
    },
};

use super::Sweep;

impl Sweep for Edge {
    type Swept = Face;

    fn sweep(
        self,
        path: impl Into<fj_math::Vector<3>>,
        tolerance: crate::algorithms::Tolerance,
        color: fj_interop::mesh::Color,
    ) -> Self::Swept {
        let path = path.into();

        let is_sweep_along_negative_direction =
            path.dot(&Vector::from([0., 0., 1.])) < Scalar::ZERO;

        if let Some(vertices) = self.vertices().get() {
            let face = create_non_continuous_side_face(
                path,
                is_sweep_along_negative_direction,
                vertices.map(|vertex| *vertex.global()),
                color,
            );
            return face;
        }

        create_continuous_side_face(self, path, tolerance, color)
    }
}

fn create_non_continuous_side_face(
    path: Vector<3>,
    is_sweep_along_negative_direction: bool,
    vertices_bottom: [GlobalVertex; 2],
    color: Color,
) -> Face {
    let vertices = {
        let vertices_top = vertices_bottom.map(|vertex| {
            let position = vertex.position() + path;
            GlobalVertex::from_position(position)
        });

        let [[a, b], [c, d]] = [vertices_bottom, vertices_top];

        if is_sweep_along_negative_direction {
            [b, a, c, d]
        } else {
            [a, b, d, c]
        }
    };

    let surface = {
        let [a, b, _, c] = vertices.map(|vertex| vertex.position());
        Surface::plane_from_points([a, b, c])
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

                Curve::new(local, global)
            };

            let vertices = VerticesOfEdge::from_vertices([
                Vertex::new(Point::from([0.]), a.1),
                Vertex::new(Point::from([1.]), b.1),
            ]);

            let edge = Edge::new(curve, vertices);

            edges.push(edge);
        }

        Cycle::new(surface).with_edges(edges)
    };

    Face::new(surface).with_exteriors([cycle]).with_color(color)
}

fn create_continuous_side_face(
    edge: Edge,
    path: Vector<3>,
    tolerance: Tolerance,
    color: Color,
) -> Face {
    let translation = Transform::translation(path);

    // This is definitely the wrong surface, but it shouldn't matter. Since this
    // code will hopefully soon be gone anyway (this is the last piece of code
    // that prevents us from removing triangle representation), it hopefully
    // won't start to matter at some point either.
    let placeholder = Surface::xy_plane();

    let cycle = Cycle::new(placeholder).with_edges([edge]);
    let approx = CycleApprox::new(&cycle, tolerance);

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
