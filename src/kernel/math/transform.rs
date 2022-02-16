use super::{Point, Vector};

/// A transform
pub struct Transform(parry3d_f64::math::Isometry<f64>);

impl Transform {
    /// Transform the given point
    pub fn transform_point(&self, point: &Point<3>) -> Point<3> {
        self.0.transform_point(point)
    }

    /// Transform the given vector
    pub fn transform_vector(&self, vector: &Vector<3>) -> Vector<3> {
        self.0.transform_vector(vector)
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
