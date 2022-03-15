use std::collections::HashMap;

use crate::{
    kernel::{
        shape::{Handle, Shape},
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
    mut source: Shape,
    path: Vector<3>,
    tolerance: Scalar,
    color: [u8; 4],
) -> Shape {
    let mut target = source.clone();

    let translation = Transform::translation(path);

    let mut source_to_top = Relation::new();

    // Create the new vertices.
    for vertex_source in source.topology().vertices() {
        let point = target
            .geometry()
            .add_point(vertex_source.get().point() + path);
        let vertex = target.topology().add_vertex(Vertex { point }).unwrap();
        source_to_top.vertices.insert(vertex_source, vertex);
    }

    // Create the new edges.
    for edge_source in source.topology().edges() {
        let curve = target
            .geometry()
            .add_curve(edge_source.get().curve().transform(&translation));

        let vertices = edge_source.get().vertices.clone().map(|vs| {
            vs.map(|vertex_source| {
                // Can't panic, as long as the original shape is valid. We've
                // added all its vertices to `vertices`.
                source_to_top.vertices.get(&vertex_source).unwrap().clone()
            })
        });

        let edge = target
            .topology()
            .add_edge(Edge { curve, vertices })
            .unwrap();
        source_to_top.edges.insert(edge_source, edge);
    }

    // Create the new cycles.
    for cycle_source in source.topology().cycles() {
        let edges = cycle_source
            .get()
            .edges
            .iter()
            .map(|edge_source| {
                // Can't panic, as long as the original shape is valid. We've
                // added all its edges to `edges`.
                source_to_top.edges.get(edge_source).unwrap().clone()
            })
            .collect();

        let cycle = target.topology().add_cycle(Cycle { edges }).unwrap();
        source_to_top.cycles.insert(cycle_source, cycle);
    }

    // Create top faces.
    for face_source in source.topology().faces().values() {
        let cycles_orig = match &face_source {
            Face::Face { cycles, .. } => cycles,
            _ => {
                // Sketches are created using boundary representation, so this
                // case can't happen.
                unreachable!()
            }
        };

        let surface = target
            .geometry()
            .add_surface(face_source.surface().transform(&translation));

        let cycles = cycles_orig
            .iter()
            .map(|cycle_orig| {
                // Can't panic, as long as the original shape is valid. We've
                // added all its cycles to `cycles`.
                source_to_top.cycles.get(cycle_orig).unwrap().clone()
            })
            .collect();

        target
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

    for cycle in source.topology().cycles().values() {
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

        target
            .topology()
            .add_face(Face::Triangles(side_face))
            .unwrap();
    }

    target
}

struct Relation {
    vertices: HashMap<Handle<Vertex>, Handle<Vertex>>,
    edges: HashMap<Handle<Edge>, Handle<Edge>>,
    cycles: HashMap<Handle<Cycle>, Handle<Cycle>>,
}

impl Relation {
    fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            cycles: HashMap::new(),
        }
    }
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
