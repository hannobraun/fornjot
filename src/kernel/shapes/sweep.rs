use std::f64::consts::PI;

use nalgebra::vector;
use parry3d_f64::math::Isometry;

use crate::{
    debug::DebugInfo,
    kernel::{
        approximation::Approximation,
        topology::{
            edges::Edges,
            faces::{Face, Faces},
            vertices::Vertices,
            Shape,
        },
    },
    math::{Aabb, Scalar, Transform},
};

use super::ToShape;

impl ToShape for fj::Sweep {
    fn to_shape(&self, tolerance: Scalar, debug_info: &mut DebugInfo) -> Shape {
        let rotation = Isometry::rotation(vector![PI, 0., 0.]).into();
        let translation = Isometry::translation(0.0, 0.0, self.length).into();

        let mut bottom_faces = Vec::new();
        let mut top_faces = Vec::new();
        let mut side_faces = Vec::new();

        let original_faces = self.shape.to_shape(tolerance, debug_info).faces;
        for face in original_faces.0 {
            // This only works for faces that are symmetric to the x-axis.
            //
            // See issue:
            // https://github.com/hannobraun/Fornjot/issues/230
            bottom_faces.push(face.clone().transform(&rotation));

            top_faces.push(face.transform(&translation));
        }

        for cycle in self.shape.edges().cycles {
            let approx = Approximation::for_cycle(&cycle, tolerance);

            // This will only work correctly, if the cycle consists of one edge.
            // If there are more, this will create some kind of weird face
            // chimera, a single face to represent all the side faces.

            let mut quads = Vec::new();
            for segment in approx.segments {
                let [v0, v1] = segment.points();
                let [v3, v2] = {
                    let segment = Transform::translation(0., 0., self.length)
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

        let faces = Faces(faces);

        Shape { faces }
    }

    fn bounding_volume(&self) -> Aabb<3> {
        let mut aabb = self.shape.bounding_volume();
        aabb.max.z = self.length.into();
        aabb
    }

    fn edges(&self) -> Edges {
        todo!()
    }

    fn vertices(&self) -> Vertices {
        todo!()
    }
}
