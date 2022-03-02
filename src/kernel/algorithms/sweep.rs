use std::f64::consts::PI;

use nalgebra::vector;
use parry3d_f64::math::Isometry;

use crate::{
    kernel::topology::{
        faces::{Face, Faces},
        Shape,
    },
    math::{Scalar, Transform, Vector},
};

use super::{approximation::Approximation, transform::transform_face};

/// Create a new shape by sweeping an existing one
pub fn sweep_shape(original: &Shape, path: f64, tolerance: Scalar) -> Shape {
    let mut shape = Shape::new();

    let rotation = Isometry::rotation(vector![PI, 0., 0.]).into();
    let translation = Transform::translation(Vector::from([0.0, 0.0, path]));

    let mut bottom_faces = Vec::new();
    let mut top_faces = Vec::new();
    let mut side_faces = Vec::new();

    for face in &original.faces.0 {
        // This only works for faces that are symmetric to the x-axis.
        //
        // See issue:
        // https://github.com/hannobraun/Fornjot/issues/230
        bottom_faces.push(transform_face(face, &rotation, &mut shape));

        top_faces.push(transform_face(face, &translation, &mut shape));
    }

    for cycle in &original.edges.cycles {
        let approx = Approximation::for_cycle(cycle, tolerance);

        // This will only work correctly, if the cycle consists of one edge. If
        // there are more, this will create some kind of weird face chimera, a
        // single face to represent all the side faces.

        let mut quads = Vec::new();
        for segment in approx.segments {
            let [v0, v1] = segment.points();
            let [v3, v2] = {
                let segment =
                    Transform::translation(Vector::from([0., 0., path]))
                        .transform_segment(&segment);
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

    let mut faces = Vec::new();
    faces.extend(bottom_faces);
    faces.extend(top_faces);
    faces.extend(side_faces);

    shape.faces = Faces(faces);

    shape
}
