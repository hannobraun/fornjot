use super::{Aabb, Point, Triangle, Vector};

/// A transform
pub struct Transform(parry3d_f64::math::Isometry<f64>);

impl Transform {
    /// Transform the given point
    pub fn transform_point(&self, point: &Point<3>) -> Point<3> {
        self.0.transform_point(point)
    }

    /// Transform the given vector
    pub fn transform_vector(&self, vector: &Vector<3>) -> Vector<3> {
        Vector::from(self.0.transform_vector(&vector.to_na()))
    }

    /// Transform the given triangle
    pub fn transform_triangle(&self, triangle: &Triangle) -> Triangle {
        Triangle {
            a: self.transform_point(&triangle.a),
            b: self.transform_point(&triangle.b),
            c: self.transform_point(&triangle.c),
        }
    }

    /// Transform the given axis-aligned bounding box
    pub fn transform_aabb(&self, aabb: &Aabb) -> Aabb {
        Aabb {
            mins: self.transform_point(&aabb.mins),
            maxs: self.transform_point(&aabb.maxs),
        }
    }
}

impl From<parry3d_f64::math::Isometry<f64>> for Transform {
    fn from(isometry: parry3d_f64::math::Isometry<f64>) -> Self {
        Self(isometry)
    }
}

impl From<Transform> for parry3d_f64::math::Isometry<f64> {
    fn from(transform: Transform) -> Self {
        transform.0
    }
}

impl<'r> From<&'r Transform> for parry3d_f64::math::Isometry<f64> {
    fn from(transform: &Transform) -> Self {
        transform.0
    }
}
