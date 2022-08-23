//! Sweeping objects along a path to create new objects

use fj_interop::mesh::Color;
use fj_math::{Point, Scalar, Transform, Triangle, Vector};

use crate::{
    iter::ObjectIters,
    objects::{
        Curve, CurveKind, Cycle, Edge, Face, GlobalCurve, GlobalVertex, Shell,
        Sketch, Solid, Surface, Vertex, VerticesOfEdge,
    },
};

use super::{reverse_face, CycleApprox, Tolerance, TransformObject};

/// Sweep an object along a path to create another object
pub trait Sweep {
    /// The object that is created by sweeping the implementing object
    type Swept;

    /// Sweep the object along the given path
    fn sweep(
        self,
        path: impl Into<Vector<3>>,
        tolerance: Tolerance,
        color: Color,
    ) -> Self::Swept;
}

impl Sweep for Sketch {
    type Swept = Solid;

    fn sweep(
        self,
        path: impl Into<Vector<3>>,
        tolerance: Tolerance,
        color: Color,
    ) -> Self::Swept {
        sweep(self, path, tolerance, color)
    }
}

/// Create a solid by sweeping a sketch
pub fn sweep(
    source: Sketch,
    path: impl Into<Vector<3>>,
    tolerance: Tolerance,
    color: Color,
) -> Solid {
    let path = path.into();

    let is_sweep_along_negative_direction =
        path.dot(&Vector::from([0., 0., 1.])) < Scalar::ZERO;

    let mut target = Vec::new();

    for face in source.face_iter() {
        create_bottom_faces(
            face,
            is_sweep_along_negative_direction,
            &mut target,
        );
        create_top_face(
            face.clone(),
            path,
            is_sweep_along_negative_direction,
            &mut target,
        );

        for cycle in face.all_cycles() {
            for edge in cycle.edges() {
                if let Some(vertices) = edge.vertices().get() {
                    create_non_continuous_side_face(
                        path,
                        is_sweep_along_negative_direction,
                        vertices.map(|vertex| *vertex.global()),
                        color,
                        &mut target,
                    );
                    continue;
                }

                create_continuous_side_face(
                    *edge,
                    path,
                    tolerance,
                    color,
                    &mut target,
                );
            }
        }
    }

    let shell = Shell::new().with_faces(target);
    Solid::new().with_shells([shell])
}

fn create_bottom_faces(
    face: &Face,
    is_sweep_along_negative_direction: bool,
    target: &mut Vec<Face>,
) {
    let face = if is_sweep_along_negative_direction {
        face.clone()
    } else {
        reverse_face(face)
    };

    target.push(face);
}

fn create_top_face(
    face: Face,
    path: Vector<3>,
    is_sweep_along_negative_direction: bool,
    target: &mut Vec<Face>,
) {
    let mut face = face.translate(path);

    if is_sweep_along_negative_direction {
        face = reverse_face(&face);
    };

    target.push(face);
}

fn create_non_continuous_side_face(
    path: Vector<3>,
    is_sweep_along_negative_direction: bool,
    vertices_bottom: [GlobalVertex; 2],
    color: Color,
    target: &mut Vec<Face>,
) {
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

    let face = Face::new(surface).with_exteriors([cycle]).with_color(color);
    target.push(face);
}

fn create_continuous_side_face(
    edge: Edge,
    path: Vector<3>,
    tolerance: Tolerance,
    color: Color,
    target: &mut Vec<Face>,
) {
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

    target.push(Face::from_triangles(side_face));
}

#[cfg(test)]
mod tests {
    use fj_interop::mesh::Color;
    use fj_math::{Point, Scalar, Vector};

    use crate::{
        algorithms::Tolerance,
        iter::ObjectIters,
        objects::{Face, Sketch, Surface},
    };

    use super::Sweep;

    #[test]
    fn bottom_positive() -> anyhow::Result<()> {
        test_bottom_top(
            [0., 0., 1.],
            [[0., 0., 0.], [1., 0., 0.], [0., -1., 0.]],
            [[0., 0.], [1., 0.], [0., -1.]],
        )
    }

    #[test]
    fn bottom_negative() -> anyhow::Result<()> {
        test_bottom_top(
            [0., 0., -1.],
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
            [[0., 0.], [1., 0.], [0., 1.]],
        )
    }

    #[test]
    fn top_positive() -> anyhow::Result<()> {
        test_bottom_top(
            [0., 0., 1.],
            [[0., 0., 1.], [1., 0., 1.], [0., 1., 1.]],
            [[0., 0.], [1., 0.], [0., 1.]],
        )
    }

    #[test]
    fn top_negative() -> anyhow::Result<()> {
        test_bottom_top(
            [0., 0., -1.],
            [[0., 0., -1.], [1., 0., -1.], [0., -1., -1.]],
            [[0., 0.], [1., 0.], [0., -1.]],
        )
    }

    #[test]
    fn side_positive() -> anyhow::Result<()> {
        test_side(
            [0., 0., 1.],
            [
                [[0., 0., 0.], [1., 0., 0.], [0., 0., 1.]],
                [[1., 0., 0.], [0., 1., 0.], [1., 0., 1.]],
                [[0., 1., 0.], [0., 0., 0.], [0., 1., 1.]],
            ],
        )
    }

    #[test]
    fn side_negative() -> anyhow::Result<()> {
        test_side(
            [0., 0., -1.],
            [
                [[0., 0., 0.], [0., 1., 0.], [0., 0., -1.]],
                [[0., 1., 0.], [1., 0., 0.], [0., 1., -1.]],
                [[1., 0., 0.], [0., 0., 0.], [1., 0., -1.]],
            ],
        )
    }

    fn test_side(
        direction: impl Into<Vector<3>>,
        expected_surfaces: [[impl Into<Point<3>>; 3]; 3],
    ) -> anyhow::Result<()> {
        test(
            direction,
            expected_surfaces,
            [[0., 0.], [1., 0.], [1., 1.], [0., 1.]],
        )
    }

    fn test_bottom_top(
        direction: impl Into<Vector<3>>,
        expected_surface: [impl Into<Point<3>>; 3],
        expected_vertices: [impl Into<Point<2>>; 3],
    ) -> anyhow::Result<()> {
        test(direction, [expected_surface], expected_vertices)
    }

    fn test(
        direction: impl Into<Vector<3>>,
        expected_surfaces: impl IntoIterator<Item = [impl Into<Point<3>>; 3]>,
        expected_vertices: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> anyhow::Result<()> {
        let tolerance = Tolerance::from_scalar(Scalar::ONE)?;

        let surface = Surface::xy_plane();
        let face = Face::build(surface).polygon_from_points([
            [0., 0.],
            [1., 0.],
            [0., 1.],
        ]);
        let sketch = Sketch::new().with_faces([face]);

        let solid = sketch.sweep(direction, tolerance, Color([255, 0, 0, 255]));

        let expected_vertices: Vec<_> = expected_vertices
            .into_iter()
            .map(|vertex| vertex.into())
            .collect();

        let faces = expected_surfaces.into_iter().map(|surface| {
            let surface = Surface::plane_from_points(surface);

            Face::build(surface)
                .polygon_from_points(expected_vertices.clone())
                .into_face()
        });

        for face in faces {
            assert!(solid.face_iter().any(|f| f == &face));
        }

        Ok(())
    }
}
