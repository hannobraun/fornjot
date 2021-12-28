use nalgebra::vector;
use parry3d_f64::{bounding_volume::AABB, math::Isometry, shape::Triangle};

use crate::{
    kernel::{
        edges::Edges,
        faces::{Face, Faces},
        Shape,
    },
    math::Point,
};

impl Shape for fj::Sweep {
    fn bounding_volume(&self) -> AABB {
        let mut aabb = self.shape.bounding_volume();
        aabb.maxs.z = self.length;
        aabb
    }

    fn faces(&self, tolerance: f64) -> Faces {
        // TASK: This assumes that a 2-dimensional shape only consists of one
        //       face. I don't know if this is a reasonable assumption in
        //       general, but it certainly doesn't reflect the data structures,
        //       which allow an arbitrary number of faces in any shape.
        let mut original_face = Vec::new();
        self.shape
            .faces(tolerance)
            .triangles(tolerance, &mut original_face);

        let bottom_face = original_face
            .iter()
            .map(|triangle| {
                // Change triangle direction, as the bottom of the sweep points
                // down, while the original face pointed up.
                Triangle::new(triangle.a, triangle.c, triangle.b)
            })
            .collect();

        let top_face = original_face
            .iter()
            .map(|triangle| {
                triangle.transformed(&Isometry::translation(
                    0.0,
                    0.0,
                    self.length,
                ))
            })
            .collect();

        let segments = self.shape.edges().approx_segments(tolerance);

        let mut quads = Vec::new();
        for segment in segments {
            let [v0, v1] = [segment.a, segment.b];
            let [v3, v2] = {
                let segment =
                    segment.transformed(vector![0.0, 0.0, self.length]);
                [segment.a, segment.b]
            };

            quads.push([v0, v1, v2, v3]);
        }

        let mut side_face = Vec::new();
        for [v0, v1, v2, v3] in quads {
            side_face.push([v0, v1, v2].into());
            side_face.push([v0, v2, v3].into());
        }

        Faces(vec![
            Face::Triangles(bottom_face),
            Face::Triangles(top_face),
            Face::Triangles(side_face),
        ])
    }

    fn edges(&self) -> Edges {
        // TASK: Implement.
        todo!()
    }

    fn vertices(&self) -> Vec<Point> {
        // TASK: Implement.
        todo!()
    }
}
