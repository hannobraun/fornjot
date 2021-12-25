use parry3d_f64::{bounding_volume::AABB, math::Isometry};

use crate::{
    kernel::{edges::Edges, faces::Faces, Shape},
    math::{Point, Vector},
};

impl Shape for fj::Transform {
    fn bounding_volume(&self) -> AABB {
        self.shape.bounding_volume().transform_by(&isometry(self))
    }

    fn faces(&self, tolerance: f64) -> Faces {
        let faces = self.shape.faces(tolerance);
        let isometry = isometry(self);

        let mut triangles = faces.triangles();
        for triangle in &mut triangles {
            *triangle = triangle.transformed(&isometry);
        }

        Faces::Triangles(triangles)
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
