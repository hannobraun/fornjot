use std::collections::HashMap;

use crate::{
    kernel::{
        shape::Shape,
        topology::{
            edges::{Cycle, Edge},
            faces::Face,
            vertices::Vertex,
        },
    },
    math::{Scalar, Transform, Triangle, Vector},
};

use super::approximation::Approximation;

/// Create a new shape by sweeping an existing one
pub fn sweep_shape(
    mut shape_orig: Shape,
    path: Vector<3>,
    tolerance: Scalar,
    color: [u8; 4],
) -> Shape {
    let mut shape = shape_orig.clone();

    let translation = Transform::translation(path);

    let mut side_faces = Vec::new();

    // Create the new vertices.
    let mut vertices = HashMap::new();
    for vertex_orig in shape_orig.topology().vertices() {
        let point =
            shape.geometry().add_point(vertex_orig.get().point() + path);
        let vertex = shape.topology().add_vertex(Vertex { point }).unwrap();
        vertices.insert(vertex_orig, vertex);
    }

    // Create the new edges.
    let mut edges = HashMap::new();
    for edge_orig in shape_orig.topology().edges() {
        let curve = shape
            .geometry()
            .add_curve(edge_orig.get().curve().transform(&translation));

        let vertices = edge_orig.get().vertices.clone().map(|vs| {
            vs.map(|vertex_orig| {
                // Can't panic, as long as the original shape is valid. We've
                // added all its vertices to `vertices`.
                vertices.get(&vertex_orig).unwrap().clone()
            })
        });

        let edge = shape.topology().add_edge(Edge { curve, vertices }).unwrap();
        edges.insert(edge_orig, edge);
    }

    // Create the new cycles.
    let mut cycles = HashMap::new();
    for cycle_orig in shape_orig.topology().cycles() {
        let edges = cycle_orig
            .get()
            .edges
            .iter()
            .map(|edge_orig| {
                // Can't panic, as long as the original shape is valid. We've
                // added all its edges to `edges`.
                edges.get(edge_orig).unwrap().clone()
            })
            .collect();

        let cycle = shape.topology().add_cycle(Cycle { edges }).unwrap();
        cycles.insert(cycle_orig, cycle);
    }

    // Create top faces.
    for face_orig in shape_orig.topology().faces().values() {
        let cycles_orig = match &face_orig {
            Face::Face { cycles, .. } => cycles,
            _ => {
                // Sketches are created using boundary representation, so this
                // case can't happen.
                unreachable!()
            }
        };

        let surface = shape
            .geometry()
            .add_surface(face_orig.surface().transform(&translation));

        let cycles = cycles_orig
            .iter()
            .map(|cycle_orig| {
                // Can't panic, as long as the original shape is valid. We've
                // added all its cycles to `cycles`.
                cycles.get(cycle_orig).unwrap().clone()
            })
            .collect();

        shape
            .topology()
            .add_face(Face::Face {
                surface,
                cycles,
                color,
            })
            .unwrap();
    }

    // We could use `vertices` to create the side edges and faces here, but the
    // side walls are created below, in triangle representation.

    for cycle in shape_orig.topology().cycles().values() {
        let approx = Approximation::for_cycle(&cycle, tolerance);

        // This will only work correctly, if the cycle consists of one edge. If
        // there are more, this will create some kind of weird face chimera, a
        // single face to represent all the side faces.

        let mut quads = Vec::new();
        for segment in approx.segments {
            let [v0, v1] = segment.points();
            let [v3, v2] = {
                let segment =
                    Transform::translation(path).transform_segment(&segment);
                segment.points()
            };

            quads.push([v0, v1, v2, v3]);
        }

        let mut side_face: Vec<Triangle<3>> = Vec::new();
        for [v0, v1, v2, v3] in quads {
            side_face.push([v0, v1, v2].into());
            side_face.push([v0, v2, v3].into());
        }

        // FIXME: We probably want to allow the use of custom colors for the "walls" of the swept
        // object.
        for s in side_face.iter_mut() {
            s.set_color(color);
        }

        side_faces.push(Face::Triangles(side_face));
    }

    for face in side_faces {
        shape.topology().add_face(face).unwrap();
    }

    shape
}

#[cfg(test)]
mod tests {
    use crate::{
        kernel::{
            geometry::{surfaces::Swept, Surface},
            shape::{Handle, Shape},
            topology::{edges::Cycle, faces::Face, vertices::Vertex},
        },
        math::{Point, Scalar, Vector},
    };

    use super::sweep_shape;

    #[test]
    fn sweep() {
        let sketch = Triangle::new([[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]]);

        let mut swept = sweep_shape(
            sketch.shape,
            Vector::from([0., 0., 1.]),
            Scalar::from_f64(0.),
            [255, 0, 0, 255],
        );

        let bottom_face = sketch.face.get().clone();
        let top_face =
            Triangle::new([[0., 0., 1.], [1., 0., 1.], [0., 1., 1.]])
                .face
                .get()
                .clone();

        let mut contains_bottom_face = false;
        let mut contains_top_face = false;

        for face in swept.topology().faces() {
            if matches!(&*face.get(), Face::Face { .. }) {
                if face.get().clone() == bottom_face {
                    contains_bottom_face = true;
                }
                if face.get().clone() == top_face {
                    contains_top_face = true;
                }
            }
        }

        assert!(contains_bottom_face);
        assert!(contains_top_face);

        // Side faces are not tested, as those use triangle representation. The
        // plan is to start testing them, as they are transitioned to b-rep.
    }

    pub struct Triangle {
        shape: Shape,
        face: Handle<Face>,
    }

    impl Triangle {
        fn new([a, b, c]: [impl Into<Point<3>>; 3]) -> Self {
            let mut shape = Shape::new();

            let a = shape.geometry().add_point(a.into());
            let b = shape.geometry().add_point(b.into());
            let c = shape.geometry().add_point(c.into());

            let a = shape.topology().add_vertex(Vertex { point: a }).unwrap();
            let b = shape.topology().add_vertex(Vertex { point: b }).unwrap();
            let c = shape.topology().add_vertex(Vertex { point: c }).unwrap();

            let ab = shape
                .topology()
                .add_line_segment([a.clone(), b.clone()])
                .unwrap();
            let bc = shape
                .topology()
                .add_line_segment([b.clone(), c.clone()])
                .unwrap();
            let ca = shape
                .topology()
                .add_line_segment([c.clone(), a.clone()])
                .unwrap();

            let cycles = shape
                .topology()
                .add_cycle(Cycle {
                    edges: vec![ab, bc, ca],
                })
                .unwrap();

            let surface = shape.geometry().add_surface(Surface::Swept(
                Swept::plane_from_points(
                    [a, b, c].map(|vertex| vertex.get().point()),
                ),
            ));
            let abc = Face::Face {
                surface,
                cycles: vec![cycles],
                color: [255, 0, 0, 255],
            };

            let face = shape.topology().add_face(abc).unwrap();

            Self { shape, face }
        }
    }
}
