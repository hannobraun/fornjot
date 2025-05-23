use std::ops;

use super::{Aabb, LineSegment, Point, Triangle, Vector};

/// An affine transform
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Transform {
    /// # The [`nalgebra`] transform that does the actual work
    pub inner: nalgebra::Transform<f64, nalgebra::TAffine, 3>,
}

impl Transform {
    /// Construct an identity transform
    pub fn identity() -> Self {
        Self {
            inner: nalgebra::Transform::identity(),
        }
    }

    /// Construct a translation
    pub fn translation(offset: impl Into<Vector<3>>) -> Self {
        let offset = offset.into();

        Self {
            inner: nalgebra::Transform::from_matrix_unchecked(
                nalgebra::OMatrix::new_translation(&offset.to_na()),
            ),
        }
    }

    /// Construct a rotation
    ///
    /// The direction of the vector defines the rotation axis. Its length
    /// defines the angle of the rotation.
    pub fn rotation(axis_angle: impl Into<Vector<3>>) -> Self {
        let axis_angle = axis_angle.into();

        Self {
            inner: nalgebra::Transform::from_matrix_unchecked(
                nalgebra::OMatrix::<_, nalgebra::Const<4>, _>::new_rotation(
                    axis_angle.to_na(),
                ),
            ),
        }
    }

    /// Construct a scaling
    pub fn scale(scaling_factor: f64) -> Self {
        Self {
            inner: nalgebra::Transform::from_matrix_unchecked(
                nalgebra::OMatrix::new_scaling(scaling_factor),
            ),
        }
    }

    /// # Extract the "right" vector from the rotational component
    pub fn right(&self) -> Vector<3> {
        let d = self.data();
        Vector::from([d[0], d[1], d[2]])
    }

    /// # Extract the "up" vector from the rotational component
    pub fn up(&self) -> Vector<3> {
        let d = self.data();
        Vector::from([d[4], d[5], d[6]])
    }

    /// Transform the given point
    pub fn transform_point(&self, point: &Point<3>) -> Point<3> {
        Point::from(self.inner.transform_point(&point.to_na()))
    }

    /// Inverse transform given point
    pub fn inverse_transform_point(&self, point: &Point<3>) -> Point<3> {
        Point::from(self.inner.inverse_transform_point(&point.to_na()))
    }

    /// Transform the given vector
    pub fn transform_vector(&self, vector: &Vector<3>) -> Vector<3> {
        Vector::from(self.inner.transform_vector(&vector.to_na()))
    }

    /// Transform the given segment
    pub fn transform_segment(
        &self,
        segment: &LineSegment<3>,
    ) -> LineSegment<3> {
        let [a, b] = &segment.points;
        LineSegment::from([self.transform_point(a), self.transform_point(b)])
    }

    /// Transform the given triangle
    pub fn transform_triangle(&self, triangle: &Triangle<3>) -> Triangle<3> {
        let [a, b, c] = &triangle.points;
        Triangle::from([
            self.transform_point(a),
            self.transform_point(b),
            self.transform_point(c),
        ])
    }

    /// Inverse transform
    pub fn inverse(&self) -> Self {
        Self {
            inner: self.inner.inverse(),
        }
    }

    /// Transpose transform
    pub fn transpose(&self) -> Self {
        Self {
            inner: nalgebra::Transform::from_matrix_unchecked(
                self.inner.to_homogeneous().transpose(),
            ),
        }
    }

    /// Transform the given axis-aligned bounding box
    pub fn transform_aabb(&self, aabb: &Aabb<3>) -> Aabb<3> {
        Aabb {
            min: self.transform_point(&aabb.min),
            max: self.transform_point(&aabb.max),
        }
    }

    /// Exposes the data of this Transform as a slice of f64.
    pub fn data(&self) -> &[f64] {
        self.inner.matrix().data.as_slice()
    }

    /// Extract the rotation component of this transform
    pub fn extract_rotation(&self) -> Self {
        Self {
            inner: nalgebra::Transform::from_matrix_unchecked(
                self.inner
                    .matrix()
                    .fixed_resize::<3, 3>(0.)
                    .to_homogeneous(),
            ),
        }
    }

    /// Extract the translation component of this transform
    pub fn extract_translation(&self) -> Self {
        *self * self.extract_rotation().inverse()
    }
}

impl ops::Mul<Self> for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            inner: self.inner.mul(rhs.inner),
        }
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{Scalar, Vector};

    use super::Transform;

    #[test]
    fn extract_rotation_translation() {
        let rotation =
            Transform::rotation(Vector::unit_z() * (Scalar::PI / 2.));
        let translation = Transform::translation([1., 2., 3.]);

        assert_abs_diff_eq!(
            (translation * rotation).extract_rotation().data(),
            rotation.data(),
            epsilon = 1e-8,
        );

        assert_abs_diff_eq!(
            (translation * rotation).extract_translation().data(),
            translation.data(),
            epsilon = 1e-8,
        );

        assert_abs_diff_eq!(
            (rotation * translation).extract_rotation().data(),
            rotation.data(),
            epsilon = 1e-8,
        );

        assert_abs_diff_eq!(
            (rotation * translation).extract_translation().data(),
            Transform::translation([-2., 1., 3.]).data(),
            epsilon = 1e-8,
        );
    }
}
