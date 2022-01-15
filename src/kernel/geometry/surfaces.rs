use crate::math::{Point, Vector};

/// A two-dimensional shape
#[derive(Debug, PartialEq)]
pub enum Surface {
    /// A plane
    ///
    /// For the time being, this is always going to be the x-y plane. Making
    /// this code more flexible is subject of ongoing work.
    Plane,
}

impl Surface {
    /// Construct a `Surface` that represents to x-y plane
    pub fn x_y_plane() -> Self {
        Self::Plane
    }

    /// Convert a point in model coordinates to surface coordinates
    pub fn point_model_to_surface(&self, point: Point<3>) -> Point<2> {
        match self {
            Self::Plane => {
                if point.z != 0. {
                    panic!("Point {:?} is not in surface {:?}", point, self);
                }

                Point::from([point.x, point.y])
            }
        }
    }

    /// Convert a point in surface coordinates to model coordinates
    pub fn point_surface_to_model(&self, point: Point<2>) -> Point<3> {
        let coords = self.vector_surface_to_model(point.coords);
        Point { coords }
    }

    /// Convert a vector in surface coordinates to model coordinates
    pub fn vector_surface_to_model(&self, point: Vector<2>) -> Vector<3> {
        match self {
            Self::Plane => Vector::from([point.x, point.y, 0.]),
        }
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::point;

    use super::Surface;

    #[test]
    fn test_model_to_surface_point_conversion() {
        let plane = Surface::Plane;

        let valid_model_point = point![1., 2., 0.];

        let surface_point = plane.point_model_to_surface(valid_model_point);
        assert_eq!(surface_point, point![1., 2.]);
    }
}
