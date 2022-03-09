use std::collections::HashMap;

use crate::{
    kernel::{
        shape::Shape,
        topology::{
            edges::{Cycle, Edge},
            faces::Face,
        },
    },
    math::{Scalar, Transform, Vector},
};

use super::approximation::Approximation;

/// Create a new shape by sweeping an existing one
pub fn sweep_shape(
    mut shape_orig: Shape,
    path: Vector<3>,
    tolerance: Scalar,
) -> Shape {
    let mut shape = shape_orig.clone();

    let translation = Transform::translation(path);

    let mut side_faces = Vec::new();

    // Create the new vertices.
    let mut vertices = HashMap::new();
    for vertex_orig in shape_orig.vertices().all() {
        let vertex = shape.vertices().add(vertex_orig.point + path);
        vertices.insert(vertex_orig, vertex);
    }

    // Create the new edges.
    let mut edges = HashMap::new();
    for edge_orig in shape_orig.edges().all() {
        let curve = shape.curves().add(edge_orig.curve.transform(&translation));

        let vertices = edge_orig.vertices.clone().map(|vs| {
            vs.map(|vertex_orig| {
                // Can't panic, as long as the original shape is valid. We've
                // added all its vertices to `vertices`.
                vertices.get(&vertex_orig).unwrap().clone()
            })
        });

        let edge = shape.edges().add(Edge { curve, vertices });
        edges.insert(edge_orig, edge);
    }

    // Create the new cycles.
    let mut cycles = HashMap::new();
    for cycle_orig in shape_orig.cycles().all() {
        let edges = cycle_orig
            .edges
            .iter()
            .map(|edge_orig| {
                // Can't panic, as long as the original shape is valid. We've
                // added all its edges to `edges`.
                edges.get(edge_orig).unwrap().clone()
            })
            .collect();

        let cycle = shape.cycles().add(Cycle { edges });
        cycles.insert(cycle_orig, cycle);
    }

    // Create top faces.
    for face_orig in shape_orig.faces().all() {
        let (surface_orig, cycles_orig) = match &*face_orig {
            Face::Face { surface, cycles } => (surface, cycles),
            _ => {
                // Sketches are created using boundary representation, so this
                // case can't happen.
                unreachable!()
            }
        };

        let surface =
            shape.surfaces().add(surface_orig.transform(&translation));

        let cycles = cycles_orig
            .iter()
            .map(|cycle_orig| {
                // Can't panic, as long as the original shape is valid. We've
                // added all its cycles to `cycles`.
                cycles.get(cycle_orig).unwrap().clone()
            })
            .collect();

        shape.faces().add(Face::Face { surface, cycles });
    }

    // We could use `vertices` to create the side edges and faces here, but the
    // side walls are created below, in triangle representation.

    for cycle in shape_orig.cycles().all() {
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

        let mut side_face = Vec::new();
        for [v0, v1, v2, v3] in quads {
            side_face.push([v0, v1, v2].into());
            side_face.push([v0, v2, v3].into());
        }

        side_faces.push(Face::Triangles(side_face));
    }

    for face in side_faces {
        shape.faces().add(face);
    }

    shape
}

#[cfg(test)]
mod tests {
    use crate::{
        kernel::{
            geometry::{surfaces::Swept, Surface},
            shape::{handle::Handle, Shape},
            topology::{edges::Cycle, faces::Face},
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
        );

        let bottom_face = sketch.face;
        let top_face =
            Triangle::new([[0., 0., 1.], [1., 0., 1.], [0., 1., 1.]]).face;

        assert!(swept.faces().contains(&bottom_face));
        assert!(swept.faces().contains(&top_face));

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

            let a = shape.vertices().add(a.into());
            let b = shape.vertices().add(b.into());
            let c = shape.vertices().add(c.into());

            let ab = shape.edges().add_line_segment([a.clone(), b.clone()]);
            let bc = shape.edges().add_line_segment([b.clone(), c.clone()]);
            let ca = shape.edges().add_line_segment([c.clone(), a.clone()]);

            let cycles = shape.cycles().add(Cycle {
                edges: vec![ab, bc, ca],
            });

            let surface =
                shape
                    .surfaces()
                    .add(Surface::Swept(Swept::plane_from_points(
                        [a, b, c].map(|vertex| vertex.point),
                    )));
            let abc = Face::Face {
                surface,
                cycles: vec![cycles],
            };

            let face = shape.faces().add(abc);

            Self { shape, face }
        }
    }
}
