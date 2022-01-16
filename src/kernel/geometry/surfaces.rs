use nalgebra::{point, vector, UnitQuaternion};
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
            v: vector![1., 0., 0.],
            w: vector![0., 1., 0.],
        })
    }

    /// Transform the surface
    pub fn transform(&mut self, transform: &Isometry<f64>) {
        match self {
            Self::Plane(plane) => plane.transform(transform),
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
            Self::Plane(Plane { origin, v, w }) => {
                // This method doesn't support any rotated planes yet.
                assert_eq!(v, &vector![1., 0., 0.]);
                assert_eq!(w, &vector![0., 1., 0.]);

                point![point.x, point.y, 0.] + origin.coords
            }
        }
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_surface_to_model(&self, point: Vector<2>) -> Vector<3> {
        match self {
            Self::Plane(Plane { origin: _, v, w }) => {
                // This method doesn't support any rotated planes yet.
                assert_eq!(v, &vector![1., 0., 0.]);
                assert_eq!(w, &vector![0., 1., 0.]);

                Vector::from([point.x, point.y, 0.])
            }
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
    /// Must not be parallel to `w`.
    pub v: Vector<3>,

    /// Second direction that defines the plane orientation
    ///
    /// It might be most reasonable, if this were a unit vector that is
    /// orthogonal to `w`. As an experiment, this isn't required right now,
    /// to allow for the definition of interesting coordinate systems. It's
    /// unclear how well all algorithms will handle those though.
    ///
    /// Must not be parallel to `v`.
    pub w: Vector<3>,
}

impl Plane {
    /// Transform the plane
    pub fn transform(&mut self, transform: &Isometry<f64>) {
        // The plane representation is still too limited to support
        // rotations.
        assert!(transform.rotation == UnitQuaternion::identity());

        self.origin = transform.transform_point(&self.origin);
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
        // This method doesn't support any rotated planes yet.
        assert_eq!(self.v, vector![1., 0., 0.]);
        assert_eq!(self.w, vector![0., 1., 0.]);

        if point.z != self.origin.z {
            return Err(());
        }

        Ok(point.xy() - self.origin.xy().coords)
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::{point, vector, UnitQuaternion};
    use parry3d_f64::math::{Isometry, Translation};

    use super::{Plane, Surface};

    #[test]
    fn test_transform() {
        let mut plane = Surface::Plane(Plane {
            origin: point![1., 2., 3.],
            v: vector![1., 0., 0.],
            w: vector![0., 1., 0.],
        });

        plane.transform(&Isometry::from_parts(
            Translation::from([2., 4., 6.]),
            UnitQuaternion::identity(),
        ));

        assert_eq!(
            plane,
            Surface::Plane(Plane {
                origin: point![3., 6., 9.],
                v: vector![1., 0., 0.],
                w: vector![0., 1., 0.],
            })
        );
    }

    #[test]
    fn test_model_to_surface_point_conversion() {
        let plane = Surface::Plane(Plane {
            origin: point![1., 2., 3.],
            v: vector![1., 0., 0.],
            w: vector![0., 1., 0.],
        });

        let valid_model_point = point![2., 4., 3.];
        let invalid_model_point = point![2., 4., 6.];

        assert_eq!(
            plane.point_model_to_surface(valid_model_point),
            Ok(point![1., 2.]),
        );
        assert_eq!(plane.point_model_to_surface(invalid_model_point), Err(()));
    }

    #[test]
    fn test_surface_to_model_point_conversion() {
        let plane = Surface::Plane(Plane {
            origin: point![1., 2., 3.],
            v: vector![1., 0., 0.],
            w: vector![0., 1., 0.],
        });

        assert_eq!(
            plane.point_surface_to_model(point![2., 4.]),
            point![3., 6., 3.],
        );
    }

    #[test]
    fn test_surface_to_model_vector_conversion() {
        let plane = Surface::Plane(Plane {
            origin: point![1., 2., 3.],
            v: vector![1., 0., 0.],
            w: vector![0., 1., 0.],
        });

        assert_eq!(
            plane.vector_surface_to_model(vector![2., 4.]),
            vector![2., 4., 0.],
        );
    }
}
