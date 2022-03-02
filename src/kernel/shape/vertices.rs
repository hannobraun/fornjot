use kiddo::distance::squared_euclidean;
use tracing::warn;

use crate::{
    kernel::topology::vertices::Vertex,
    math::{Point, Scalar},
};

use super::{handle::Handle, VerticesInner};

/// The vertices of a shape
pub struct Vertices<'r> {
    pub(super) min_distance: Scalar,
    pub(super) vertices: &'r mut VerticesInner,
}

impl Vertices<'_> {
    /// Create a vertex
    ///
    /// The caller must make sure to uphold all rules regarding vertex
    /// uniqueness.
    ///
    /// # Implementation note
    ///
    /// This method is the only means to create `Vertex` instances, outside of
    /// unit tests. That puts this method is in a great position to enforce
    /// vertex uniqueness rules, instead of requiring the user to uphold those.
    pub fn create(&mut self, point: Point<3>) -> Vertex {
        let handle = Handle::new(point);

        // Make sure the new vertex is a minimum distance away from all existing
        // vertices. This minimum distance is defined to be half a µm, which
        // should provide more than enough precision for common use cases, while
        // being large enough to catch all invalid cases.
        match self.vertices.nearest_one(&point.into(), &squared_euclidean) {
            Ok((distance_squared, existing)) => {
                if distance_squared < self.min_distance * self.min_distance {
                    let existing = existing.get();

                    warn!(
                        "Invalid vertex: {point:?}; \
                        identical vertex at {existing:?}",
                    );
                }
            }
            Err(kiddo::ErrorKind::Empty) => {
                // No other vertices means no change of the new one being
                // invalid.
            }
            Err(err) => {
                panic!("Error during vertex validation: {err:?}");
            }
        }

        self.vertices
            .add(&point.into(), handle.inner())
            .expect("Error adding vertex");

        Vertex(handle)
    }
}

#[cfg(test)]
mod tests {
    use crate::{kernel::shape::Shape, math::Point};

    #[test]
    fn create_valid() {
        let mut shape = Shape::new();

        shape.vertices().create(Point::from([0., 0., 0.]));
        shape.vertices().create(Point::from([5e-6, 0., 0.]));
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn create_invalid() {
        let mut shape = Shape::new();

        shape.vertices().create(Point::from([0., 0., 0.]));
        shape.vertices().create(Point::from([5e-8, 0., 0.]));
    }
}
