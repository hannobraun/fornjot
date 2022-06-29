use fj_math::{Point, Scalar, Transform, Triangle, Vector};

use crate::{
    iter::ObjectIters,
    objects::{
        Curve, Cycle, CyclesInFace, Edge, Face, Surface, Vertex, VerticesOfEdge,
    },
    shape::LocalForm,
};

use super::{transform::transform_face, CycleApprox, Tolerance};

/// Create a solid by sweeping a sketch
pub fn sweep(
    source: Vec<Face>,
    path: impl Into<Vector<3>>,
    tolerance: Tolerance,
    color: [u8; 4],
) -> Vec<Face> {
    let path = path.into();

    let is_sweep_along_negative_direction =
        path.dot(&Vector::from([0., 0., 1.])) < Scalar::ZERO;

    let mut target = Vec::new();

    for face in source.face_iter() {
        create_bottom_faces(
            &face,
            is_sweep_along_negative_direction,
            &mut target,
        );
        create_top_face(
            &face,
            path,
            is_sweep_along_negative_direction,
            &mut target,
        );
    }

    for edge in source.edge_iter() {
        if let Some(vertices) = edge.vertices() {
            create_non_continuous_side_face(
                path,
                is_sweep_along_negative_direction,
                vertices,
                color,
                &mut target,
            );
            continue;
        }

        create_continuous_side_face(edge, path, tolerance, color, &mut target);
    }

    target
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
    face: &Face,
    path: Vector<3>,
    is_sweep_along_negative_direction: bool,
    target: &mut Vec<Face>,
) {
    let translation = Transform::translation(path);
    let mut face = transform_face(face, &translation);

    if is_sweep_along_negative_direction {
        face = reverse_face(&face);
    };

    target.push(face);
}

fn reverse_face(face: &Face) -> Face {
    let face = match face {
        Face::Face(face) => face,
        Face::Triangles(_) => {
            panic!("Reversing tri-rep faces is not supported")
        }
    };

    let surface = face.surface().reverse();

    let exteriors = reverse_local_coordinates_in_cycle(&face.exteriors);
    let interiors = reverse_local_coordinates_in_cycle(&face.interiors);

    Face::new(
        surface,
        exteriors.as_local_form().cloned(),
        interiors.as_local_form().cloned(),
        face.color,
    )
}

fn reverse_local_coordinates_in_cycle(cycles: &CyclesInFace) -> CyclesInFace {
    let cycles = cycles.as_local_form().map(|cycle| {
        let edges = cycle
            .local()
            .edges
            .iter()
            .map(|edge| {
                let curve = LocalForm::new(
                    // This is wrong. We have reversed the direction of the
                    // surface, thereby modifying its coordinate system. So we
                    // can't just use the local form of the curve, which is
                    // expressed in surface coordinates, as-is.
                    //
                    // This is a coherence issue, but since coherence validation
                    // is not complete, and the whole local form stuff is still
                    // a work in progress, this doesn't lead to any observable
                    // bugs.
                    *edge.local().curve.local(),
                    edge.local().curve.canonical(),
                );
                let vertices = edge.local().vertices.clone().map(|vertex| {
                    LocalForm::new(*vertex.local(), vertex.canonical())
                });
                let local = Edge { curve, vertices };
                LocalForm::new(local, edge.canonical())
            })
            .collect();
        let local = Cycle { edges };
        LocalForm::new(local, cycle.canonical())
    });

    CyclesInFace::new(cycles)
}

fn create_non_continuous_side_face(
    path: Vector<3>,
    is_sweep_along_negative_direction: bool,
    vertices_bottom: [Vertex; 2],
    color: [u8; 4],
    target: &mut Vec<Face>,
) {
    let vertices = {
        let vertices_top = vertices_bottom.map(|vertex| {
            let point = vertex.point + path;
            Vertex { point }
        });

        let [[a, b], [c, d]] = [vertices_bottom, vertices_top];

        if is_sweep_along_negative_direction {
            [b, a, c, d]
        } else {
            [a, b, d, c]
        }
    };

    let surface = {
        let [a, b, _, c] = vertices.map(|vertex| vertex.point);
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
                let local = Curve::line_from_points([a.0, b.0]);

                let global = [a, b].map(|vertex| vertex.1.point);
                let global = Curve::line_from_points(global);

                LocalForm::new(local, global)
            };

            let vertices = VerticesOfEdge::from_vertices([
                LocalForm::new(Point::from([0.]), a.1),
                LocalForm::new(Point::from([1.]), b.1),
            ]);

            let edge = {
                let local = Edge {
                    curve: curve.clone(),
                    vertices: vertices.clone(),
                };

                let global = Edge {
                    curve: LocalForm::canonical_only(curve.canonical()),
                    vertices,
                };

                LocalForm::new(local, global)
            };

            edges.push(edge);
        }

        let cycle = {
            let local = Cycle { edges };

            let global =
                Cycle::new(local.edges.iter().map(|edge| edge.canonical()));

            LocalForm::new(local, global)
        };

        cycle
    };

    let face = Face::new(surface, [cycle], [], color);
    target.push(face);
}

fn create_continuous_side_face(
    edge: Edge<3>,
    path: Vector<3>,
    tolerance: Tolerance,
    color: [u8; 4],
    target: &mut Vec<Face>,
) {
    let translation = Transform::translation(path);

    let cycle = Cycle::new(vec![edge]);
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

    target.push(Face::Triangles(side_face));
}

#[cfg(test)]
mod tests {
    use fj_math::{Point, Scalar, Vector};

    use crate::{
        algorithms::Tolerance,
        iter::ObjectIters,
        objects::{Face, Surface},
    };

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

        let sketch = Face::builder(Surface::xy_plane())
            .with_exterior_polygon([[0., 0.], [1., 0.], [0., 1.]])
            .build();

        let solid =
            super::sweep(vec![sketch], direction, tolerance, [255, 0, 0, 255]);

        let expected_vertices: Vec<_> = expected_vertices
            .into_iter()
            .map(|vertex| vertex.into())
            .collect();

        let faces = expected_surfaces.into_iter().map(|surface| {
            let surface = Surface::plane_from_points(surface);

            Face::builder(surface)
                .with_exterior_polygon(expected_vertices.clone())
                .build()
        });

        for face in faces {
            assert!(solid.face_iter().any(|f| f == face));
        }

        Ok(())
    }
}
