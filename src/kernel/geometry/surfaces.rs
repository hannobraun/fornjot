use crate::math::{Point, Vector};

/// A two-dimensional shape
#[derive(Debug, PartialEq)]
pub enum Surface {
    /// The X-Y plane
    ///
    /// This will be replaced with a more general plane representation in due
    /// time.
    XYPlane,
}

impl Surface {
    /// Construct a `Surface` that represents to x-y plane
    pub fn x_y_plane() -> Self {
        Self::XYPlane
    }

    /// Convert a point in model coordinates to surface coordinates
    pub fn point_model_to_surface(&self, point: Point<3>) -> Point<2> {
        match self {
            Self::XYPlane => {
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
            Self::XYPlane => Vector::from([point.x, point.y, 0.]),
        }
    }
}
