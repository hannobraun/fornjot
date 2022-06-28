use fj_math::{Point, Scalar, Transform, Triangle, Vector};

use crate::{
    iter::ObjectIters,
    objects::{
        Curve, Cycle, CyclesInFace, Edge, Face, Surface, Vertex, VerticesOfEdge,
    },
    shape::{LocalForm, Shape},
};

use super::{transform::transform_cycles, CycleApprox, Tolerance};

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
    let mut tmp = Shape::new();

    let mut surface = face.surface();

    let mut exteriors = face.brep().exteriors.clone();
    let mut interiors = face.brep().interiors.clone();

    if !is_sweep_along_negative_direction {
        surface = surface.reverse();

        exteriors = reverse_local_coordinates_in_cycle(&exteriors);
        interiors = reverse_local_coordinates_in_cycle(&interiors);
    };

    let surface = tmp.insert(surface);

    let face = Face::new(
        surface,
        exteriors.as_local_form().cloned(),
        interiors.as_local_form().cloned(),
        face.color(),
    );
    target.push(face);
}

fn create_top_face(
    face: &Face,
    path: Vector<3>,
    is_sweep_along_negative_direction: bool,
    target: &mut Vec<Face>,
) {
    let mut surface = face.surface();

    let mut exteriors = face.brep().exteriors.clone();
    let mut interiors = face.brep().interiors.clone();

    let translation = Transform::translation(path);

    surface = surface.transform(&translation);

    exteriors = transform_cycles(&exteriors, &translation);
    interiors = transform_cycles(&interiors, &translation);

    if is_sweep_along_negative_direction {
        surface = surface.reverse();

        exteriors = reverse_local_coordinates_in_cycle(&exteriors);
        interiors = reverse_local_coordinates_in_cycle(&interiors);
    };

    let mut tmp = Shape::new();
    let surface = tmp.insert(surface);

    let face = Face::new(
        surface,
        exteriors.as_local_form().cloned(),
        interiors.as_local_form().cloned(),
        face.color(),
    );
    target.push(face);
}

fn reverse_local_coordinates_in_cycle(cycles: &CyclesInFace) -> CyclesInFace {
    let cycles = cycles.as_local_form().map(|cycle| {
        let edges = cycle
            .local()
            .edges
            .iter()
            .map(|edge| {
                let curve = LocalForm::new(
                    edge.local().curve.local().reverse(),
                    edge.local().curve.canonical(),
                );
                let vertices = edge.local().vertices.clone().map(|vertex| {
                    let local = -(*vertex.local());
                    LocalForm::new(local, vertex.canonical())
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
    let mut tmp = Shape::new();

    let vertices = {
        let vertices_top = vertices_bottom.map(|vertex| {
            let point = vertex.point + path;
            Vertex { point }
        });

        let [[a, b], [c, d]] = [vertices_bottom, vertices_top];

        let vertices = if is_sweep_along_negative_direction {
            [b, a, c, d]
        } else {
            [a, b, d, c]
        };

        vertices.map(|vertex| tmp.get_handle_or_insert(vertex))
    };

    let surface = {
        let [a, b, _, c] = vertices.clone().map(|vertex| vertex.get().point);
        Surface::plane_from_points([a, b, c])
    };
    let surface = tmp.get_handle_or_insert(surface);

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

                let global = [a, b].map(|vertex| vertex.1.get().point);
                let global = Curve::line_from_points(global);
                let global = tmp.get_handle_or_insert(global);

                LocalForm::new(local, global)
            };

            let vertices = VerticesOfEdge::from_vertices([
                LocalForm::new(Point::from([0.]), a.1.clone()),
                LocalForm::new(Point::from([1.]), b.1.clone()),
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
                let global = tmp.get_handle_or_insert(global);

                LocalForm::new(local, global)
            };

            edges.push(edge);
        }

        let cycle = {
            let local = Cycle { edges };

            let global =
                Cycle::new(local.edges.iter().map(|edge| edge.canonical()));
            let global = tmp.get_handle_or_insert(global);

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

    let mut tmp = Shape::new();
    let edge = tmp.merge(edge);
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
        shape::Shape,
    };

    #[test]
    fn bottom_positive() -> anyhow::Result<()> {
        test_bottom_top(
            [0., 0., 1.],
            [[0., 0., 0.], [-1., 0., 0.], [0., 1., 0.]],
            [[0., 0.], [-1., 0.], [0., 1.]],
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
            [[0., 0., -1.], [-1., 0., -1.], [0., 1., -1.]],
            [[0., 0.], [-1., 0.], [0., 1.]],
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

        let mut shape = Shape::new();

        let sketch = Face::builder(Surface::xy_plane(), &mut shape)
            .with_exterior_polygon([[0., 0.], [1., 0.], [0., 1.]])
            .build()
            .get();

        let solid =
            super::sweep(vec![sketch], direction, tolerance, [255, 0, 0, 255]);

        let expected_vertices: Vec<_> = expected_vertices
            .into_iter()
            .map(|vertex| vertex.into())
            .collect();

        let mut shape = Shape::new();
        let faces = expected_surfaces.into_iter().map(|surface| {
            let surface = Surface::plane_from_points(surface);

            Face::builder(surface, &mut shape)
                .with_exterior_polygon(expected_vertices.clone())
                .build()
                .get()
        });

        for face in faces {
            assert!(solid.face_iter().any(|f| f == face));
        }

        Ok(())
    }
}
