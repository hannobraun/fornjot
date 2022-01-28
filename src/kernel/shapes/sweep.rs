use std::f64::consts::PI;

use nalgebra::vector;
use parry3d_f64::{bounding_volume::AABB, math::Isometry};

use crate::{
    debug::DebugInfo,
    kernel::{
        geometry,
        topology::{
            edges::Edges,
            faces::{Face, Faces},
        },
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

    fn faces(
        &self,
        tolerance: f64,
        cache: &mut geometry::Cache,
        debug_info: &mut DebugInfo,
    ) -> Faces {
        let original_faces = self.shape.faces(tolerance, cache, debug_info);

        let bottom_faces = original_faces
            .clone()
            .transform(&Isometry::rotation(vector![PI, 0., 0.]));

        let top_faces = original_faces.transform(&Isometry::translation(
            0.0,
            0.0,
            self.length,
        ));

        // This will only work correctly, if the original shape consists of one
        // edge. If there are more, this will create some kind of weird face
        // chimera, a single face to represent all the side faces.
        //
        // It'll be even worse, if the original shape consists of multiple
        // faces.
        let mut segments = Vec::new();
        self.shape
            .edges(cache)
            .approx_segments(tolerance, &mut segments);

        let mut quads = Vec::new();
        for segment in segments {
            let [v0, v1] = [segment.a, segment.b];
            let [v3, v2] = {
                let segment = segment.transformed(&Isometry::translation(
                    0.0,
                    0.0,
                    self.length,
                ));
                [segment.a, segment.b]
            };

            quads.push([v0, v1, v2, v3]);
        }

        let mut side_face = Vec::new();
        for [v0, v1, v2, v3] in quads {
            side_face.push([v0, v1, v2].into());
            side_face.push([v0, v2, v3].into());
        }

        let mut faces = Vec::new();
        faces.extend(bottom_faces.0);
        faces.extend(top_faces.0);
        faces.push(Face::Triangles(side_face));

        Faces(faces)
    }

    fn edges(&self, _: &mut geometry::Cache) -> Edges {
        todo!()
    }

    fn vertices(&self) -> Vec<Point<3>> {
        todo!()
    }
}
