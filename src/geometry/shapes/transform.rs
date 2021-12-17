use parry3d_f64::{bounding_volume::AABB, math::Isometry};

use crate::{
    geometry::{edges::Edges, faces::Faces, Shape},
    math::{Point, Vector},
};

impl Shape for fj::Transform {
    fn bounding_volume(&self) -> AABB {
        self.shape.bounding_volume().transform_by(&isometry(self))
    }

    fn faces(&self, tolerance: f64) -> Faces {
        let mut faces = self.shape.faces(tolerance);
        let isometry = isometry(self);

        for triangle in &mut faces.0 {
            *triangle = triangle.transformed(&isometry);
        }

        faces
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

fn isometry(transform: &fj::Transform) -> Isometry<f64> {
    let axis = Vector::from(transform.axis).normalize();
    Isometry::new(Vector::from(transform.offset), axis * transform.angle)
}
