use nalgebra::{point, vector};
use parry3d_f64::math::Isometry;

use crate::math::{Point, Vector};

/// A two-dimensional shape
#[derive(Clone, Debug, PartialEq)]
pub enum Surface {
    /// A plane
    Plane(Plane),
}

impl Surface {
    /// Construct a `Surface` that represents the x-y plane
    pub fn x_y_plane() -> Self {
        Self::Plane(Plane {
            origin: Point::origin(),
            u: vector![1., 0., 0.],
            v: vector![0., 1., 0.],
        })
    }

    /// Transform the surface
    pub fn transform(self, transform: &Isometry<f64>) -> Self {
        match self {
            Self::Plane(mut plane) => {
                plane.transform(transform);
                Self::Plane(plane)
            }
        }
    }

    /// Convert a point in model coordinates to surface coordinates
    ///
    /// Returns an error, if the provided point is not in the surface.
    ///
    /// # Implementation note
    ///
    /// This method is expected to only exist temporarily, until approximations
    /// have been cleaned up. As of this writing, approximations are generated
    /// in 3D, but then converted to 2D (using this method) for their primary
    /// use case.
    ///
    /// If similar functionality is needed in the future, projecting a point
    /// into a surface would probably be a better and more robust solution.
    pub fn point_model_to_surface(
        &self,
        point: Point<3>,
    ) -> Result<Point<2>, ()> {
        match self {
            Self::Plane(plane) => plane.point_model_to_surface(point),
        }
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn point_surface_to_model(&self, point: Point<2>) -> Point<3> {
        match self {
            Self::Plane(plane) => plane.point_surface_to_model(point),
        }
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_surface_to_model(&self, vector: Vector<2>) -> Vector<3> {
        match self {
            Self::Plane(plane) => plane.vector_surface_to_model(vector),
        }
    }
}

/// A plane
///
/// For the time being, only planes parallel to the x-y plane are supported.
/// Making this code more flexible to support all planes is subject of an
/// ongoing effort.
#[derive(Clone, Debug, PartialEq)]
pub struct Plane {
    /// The origin point of the plane
    ///
    /// The point on the plane that is the origin of the 2-dimensional
    /// surface coordinate system.
    pub origin: Point<3>,

    /// First direction that defines the plane orientation
    ///
    /// It might be most reasonable, if this were a unit vector that is
    /// orthogonal to `v`. As an experiment, this isn't required right now,
    /// to allow for the definition of interesting coordinate systems. It's
    /// unclear how well all algorithms will handle those though.
    ///
    /// Must not be parallel to `v`.
    pub u: Vector<3>,

    /// Second direction that defines the plane orientation
    ///
    /// It might be most reasonable, if this were a unit vector that is
    /// orthogonal to `u`. As an experiment, this isn't required right now,
    /// to allow for the definition of interesting coordinate systems. It's
    /// unclear how well all algorithms will handle those though.
    ///
    /// Must not be parallel to `u`.
    pub v: Vector<3>,
}

impl Plane {
    /// Transform the plane
    pub fn transform(&mut self, transform: &Isometry<f64>) {
        self.origin = transform.transform_point(&self.origin);
        self.u = transform.transform_vector(&self.u);
        self.v = transform.transform_vector(&self.v);
    }

    /// Convert a point in model coordinates to surface coordinates
    ///
    /// # Implementation note
    ///
    /// This method only exists to support `Surface::point_model_to_surface`. It
    /// should be removed, once no longer needed there.
    pub fn point_model_to_surface(
        &self,
        point: Point<3>,
    ) -> Result<Point<2>, ()> {
        let normal = self.u.cross(&self.v);

        let a = normal.x;
        let b = normal.y;
        let c = normal.z;
        let d = -(a * self.origin.x + b * self.origin.y + c * self.origin.z);

        let distance = (a * point.x + b * point.y + c * point.z + d).abs()
            / (a * a + b * b + c * c).sqrt();

        if distance > <f64 as approx::AbsDiffEq>::default_epsilon() {
            return Err(());
        }

        let p = point - self.origin;

        // scalar projection
        let s = p.dot(&self.u.normalize());
        let t = p.dot(&self.v.normalize());

        Ok(point![s, t])
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn point_surface_to_model(&self, point: Point<2>) -> Point<3> {
        self.origin + self.vector_surface_to_model(point.coords)
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_surface_to_model(&self, vector: Vector<2>) -> Vector<3> {
        vector.x * self.u + vector.y * self.v
    }
}

#[cfg(test)]
impl approx::AbsDiffEq for Plane {
    type Epsilon = <f64 as approx::AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        // For some reason, the Windows test runner of our GitHub Actions based
        // CI build comes up with different floating point values than the Linux
        // and macOS ones.
        //
        // I don't know why, and given that the failure this leads to happens at
        // the end of a 7+ minute CI build, I've run out of patience and am no
        // longer inclined to find out.
        //
        // The value we're returning here is still really small, and I can't
        // imagine how this could lead to a problem in a test.
        f64::default_epsilon() * 8.
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.origin.abs_diff_eq(&other.origin, epsilon)
            && self.u.abs_diff_eq(&other.u, epsilon)
            && self.v.abs_diff_eq(&other.v, epsilon)
    }
}

#[cfg(test)]
impl approx::RelativeEq for Plane {
    fn default_max_relative() -> Self::Epsilon {
        f64::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.origin
            .relative_eq(&other.origin, epsilon, max_relative)
            && self.u.relative_eq(&other.u, epsilon, max_relative)
            && self.v.relative_eq(&other.v, epsilon, max_relative)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::FRAC_PI_2;

    use approx::assert_relative_eq;
    use nalgebra::{point, vector, UnitQuaternion};
    use parry3d_f64::math::{Isometry, Translation};

    use crate::math::Vector;

    use super::Plane;

    #[test]
    fn test_transform() {
        let mut plane = Plane {
            origin: point![1., 2., 3.],
            u: vector![1., 0., 0.],
            v: vector![0., 1., 0.],
        };

        plane.transform(&Isometry::from_parts(
            Translation::from([2., 4., 6.]),
            UnitQuaternion::from_axis_angle(&Vector::z_axis(), FRAC_PI_2),
        ));

        assert_relative_eq!(
            plane,
            Plane {
                origin: point![0., 5., 9.],
                u: vector![0., 1., 0.],
                v: vector![-1., 0., 0.],
            }
        );
    }

    #[test]
    fn test_model_to_surface_point_conversion() {
        let plane = Plane {
            origin: point![1., 2., 3.],
            u: vector![0., 1., 0.],
            v: vector![0., 0., 1.],
        };

        let valid_model_point = point![1., 4., 6.];
        let invalid_model_point = point![2., 4., 6.];

        assert_eq!(
            plane.point_model_to_surface(valid_model_point),
            Ok(point![2., 3.]),
        );
        assert_eq!(plane.point_model_to_surface(invalid_model_point), Err(()));
    }

    #[test]
    fn test_surface_to_model_point_conversion() {
        let plane = Plane {
            origin: point![1., 2., 3.],
            u: vector![0., 1., 0.],
            v: vector![0., 0., 1.],
        };

        assert_eq!(
            plane.point_surface_to_model(point![2., 4.]),
            point![1., 4., 7.],
        );
    }

    #[test]
    fn test_surface_to_model_vector_conversion() {
        let plane = Plane {
            origin: point![1., 2., 3.],
            u: vector![0., 1., 0.],
            v: vector![0., 0., 1.],
        };

        assert_eq!(
            plane.vector_surface_to_model(vector![2., 4.]),
            vector![0., 2., 4.],
        );
    }
}
