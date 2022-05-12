use std::ops;

use nalgebra::Perspective3;

use crate::{Circle, Line, Scalar};

use super::{Aabb, Point, Segment, Triangle, Vector};

/// An affine transform
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Transform(nalgebra::Transform<f64, nalgebra::TAffine, 3>);

impl Transform {
    /// Construct an identity transform
    pub fn identity() -> Self {
        Self(nalgebra::Transform::identity())
    }

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

    /// Inverse transform given point
    pub fn inverse_transform_point(&self, point: &Point<3>) -> Point<3> {
        Point::from(self.0.inverse_transform_point(&point.to_na()))
    }

    /// Transform the given vector
    pub fn transform_vector(&self, vector: &Vector<3>) -> Vector<3> {
        Vector::from(self.0.transform_vector(&vector.to_na()))
    }

    /// Transform the given line
    pub fn transform_line(&self, line: &Line<3>) -> Line<3> {
        Line {
            origin: self.transform_point(&line.origin),
            direction: self.transform_vector(&line.direction),
        }
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

    /// Transform the given circle
    pub fn transform_circle(&self, circle: &Circle<3>) -> Circle<3> {
        Circle {
            center: self.transform_point(&circle.center),
            a: self.transform_vector(&circle.a),
            b: self.transform_vector(&circle.b),
        }
    }

    /// Inverse transform
    pub fn inverse(&self) -> Transform {
        Self(self.0.inverse())
    }

    /// Transpose transform
    pub fn transpose(&self) -> Transform {
        Self(nalgebra::Transform::from_matrix_unchecked(
            self.0.to_homogeneous().transpose(),
        ))
    }

    /// Project transform according to camera specfication, return data as an array.
    /// Used primarily for graphics code.
    pub fn project_to_array(
        &self,
        aspect_ratio: f64,
        fovy: f64,
        znear: f64,
        zfar: f64,
    ) -> [Scalar; 16] {
        let projection = Perspective3::new(aspect_ratio, fovy, znear, zfar);
        (projection.to_projective() * self.0)
            .matrix()
            .as_slice()
            .iter()
            .map(|f| Scalar::from(*f))
            .collect::<Vec<Scalar>>()
            .try_into()
            .unwrap()
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
        self.0.matrix().data.as_slice()
    }
}

impl ops::Mul<Self> for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.mul(rhs.0))
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use crate::{Line, Point, Scalar, Vector};

    use super::Transform;

    #[test]
    fn transform() {
        let line = Line {
            origin: Point::from([1., 0., 0.]),
            direction: Vector::from([0., 1., 0.]),
        };

        let transform = Transform::translation([1., 2., 3.])
            * Transform::rotation(Vector::unit_z() * (Scalar::PI / 2.));
        let line = transform.transform_line(&line);

        assert_abs_diff_eq!(
            line,
            Line {
                origin: Point::from([1., 3., 3.]),
                direction: Vector::from([-1., 0., 0.]),
            },
            epsilon = 1e-8,
        );
    }
}
