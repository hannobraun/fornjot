use super::{Aabb, Point, Segment, Triangle, Vector};

/// A transform
#[repr(C)]
pub struct Transform(parry3d_f64::math::Isometry<f64>);

impl Transform {
    /// Construct a translation
    pub fn translation(vector: impl Into<Vector<3>>) -> Self {
        let vector = vector.into();

        Self(parry3d_f64::math::Isometry::translation(
            vector.x.into_f64(),
            vector.y.into_f64(),
            vector.z.into_f64(),
        ))
    }

    /// Transform the given point
    pub fn transform_point(&self, point: &Point<3>) -> Point<3> {
        Point::from(self.0.transform_point(&point.to_na()))
    }

    /// Transform the given vector
    pub fn transform_vector(&self, vector: &Vector<3>) -> Vector<3> {
        Vector::from(self.0.transform_vector(&vector.to_na()))
    }

    /// Transform the given segment
    pub fn transform_segment(&self, segment: &Segment<3>) -> Segment<3> {
        let [a, b] = &segment.points();
        Segment::from([self.transform_point(a), self.transform_point(b)])
    }

    /// Transform the given triangle
    pub fn transform_triangle(&self, triangle: &Triangle<3>) -> Triangle<3> {
        let [a, b, c] = &triangle.points();
        Triangle::from([
            self.transform_point(a),
            self.transform_point(b),
            self.transform_point(c),
        ])
    }

    /// Transform the given axis-aligned bounding box
    pub fn transform_aabb(&self, aabb: &Aabb<3>) -> Aabb<3> {
        Aabb {
            min: self.transform_point(&aabb.min),
            max: self.transform_point(&aabb.max),
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
