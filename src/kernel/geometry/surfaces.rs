use crate::math::Point;

/// A two-dimensional shape
#[derive(Debug)]
pub enum Surface {
    /// The X-Y plane
    ///
    /// This will be replaced with a more general plane representation in due
    /// time.
    XYPlane,
}

impl Surface {
    /// Convert a point in surface coordinates to model coordinates
    pub fn surface_to_model(&self, point: Point<2>) -> Point<3> {
        match self {
            Surface::XYPlane => Point::from([point.x, point.y, 0.]),
        }
    }
}
