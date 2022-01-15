use nalgebra::point;
#[cfg(test)]
use nalgebra::UnitQuaternion;
#[cfg(test)]
use parry3d_f64::math::Isometry;

use crate::math::{Point, Vector};

/// A two-dimensional shape
#[derive(Clone, Debug, PartialEq)]
pub enum Surface {
    /// A plane
    ///
    /// For the time being, only planes parallel to the x-y plane are supported.
    /// Making this code more flexible to support all planes is subject of an
    /// ongoing effort.
    Plane {
        /// The origin point of the plane
        ///
        /// The point on the plane that is the origin of the 2-dimensional
        /// surface coordinate system.
        origin: Point<3>,
    },
}

impl Surface {
    /// Construct a `Surface` that represents the x-y plane
    pub fn x_y_plane() -> Self {
        Self::Plane {
            origin: Point::origin(),
        }
    }

    /// Transform the surface
    #[cfg(test)]
    pub fn transform(&mut self, transform: &Isometry<f64>) {
        match self {
            Self::Plane { origin } => {
                // The plane representation is still too limited to support
                // rotations.
                assert!(transform.rotation == UnitQuaternion::identity());

                *origin += transform.translation.vector;
            }
        }
    }

    /// Convert a point in model coordinates to surface coordinates
    ///
    /// Returns an error, if the provided point is not in the surface.
    ///
    /// # Note
    ///
    /// This method is expected to only be temporary, until the generation of
    /// approximations has been cleaned up. As of this writing, approximations
    /// are generated in 3D, but then converted to 2D (using this method) for
    /// their primary use case.
    ///
    /// If similar functionality is needed in the future, projecting a point
    /// into a surface would probably be a better and more robust solution.
    pub fn point_model_to_surface(
        &self,
        point: Point<3>,
    ) -> Result<Point<2>, ()> {
        match self {
            Self::Plane { origin } => {
                if point.z != origin.z {
                    return Err(());
                }

                Ok(point.xy() - origin.xy().coords)
            }
        }
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn point_surface_to_model(&self, point: Point<2>) -> Point<3> {
        match self {
            Self::Plane { origin } => {
                point![point.x, point.y, 0.] + origin.coords
            }
        }
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_surface_to_model(&self, point: Vector<2>) -> Vector<3> {
        match self {
            Self::Plane { origin: _ } => Vector::from([point.x, point.y, 0.]),
        }
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::{point, vector, UnitQuaternion};
    use parry3d_f64::math::{Isometry, Translation};

    use super::Surface;

    #[test]
    fn test_transform() {
        let mut plane = Surface::Plane {
            origin: point![1., 2., 3.],
        };

        plane.transform(&Isometry::from_parts(
            Translation::from([2., 4., 6.]),
            UnitQuaternion::identity(),
        ));

        assert_eq!(
            plane,
            Surface::Plane {
                origin: point![3., 6., 9.],
            }
        );
    }

    #[test]
    fn test_model_to_surface_point_conversion() {
        let plane = Surface::Plane {
            origin: point![1., 2., 3.],
        };

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
        let plane = Surface::Plane {
            origin: point![1., 2., 3.],
        };

        assert_eq!(
            plane.point_surface_to_model(point![2., 4.]),
            point![3., 6., 3.],
        );
    }

    #[test]
    fn test_surface_to_model_vector_conversion() {
        let plane = Surface::Plane {
            origin: point![1., 2., 3.],
        };

        assert_eq!(
            plane.vector_surface_to_model(vector![2., 4.]),
            vector![2., 4., 0.],
        );
    }
}
