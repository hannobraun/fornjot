use std::ops;

use super::{Aabb, Point, Segment, Triangle, Vector};

/// A transform
#[repr(C)]
pub struct Transform(nalgebra::Transform<f64, nalgebra::TGeneral, 3>);

impl Transform {
    /// Construct a translation
    pub fn translation(vector: impl Into<Vector<3>>) -> Self {
        let vector = vector.into();

        Self(nalgebra::Transform::from_matrix_unchecked(
            nalgebra::OMatrix::new_translation(&vector.to_na()),
        ))
    }

    /// Construct a rotation
    ///
    /// The direction of the vector defines the rotation axis. Its length
    /// defines the angle of the rotation.
    pub fn rotation(axis_angle: impl Into<Vector<3>>) -> Self {
        let axis_angle = axis_angle.into();

        Self(nalgebra::Transform::from_matrix_unchecked(
            nalgebra::OMatrix::<_, nalgebra::Const<4>, _>::new_rotation(
                axis_angle.to_na(),
            ),
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

impl ops::Mul<Self> for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.mul(rhs.0))
    }
}
