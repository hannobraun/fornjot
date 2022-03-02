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
    /// Logs a warning, if the vertex is not unique, meaning if another vertex
    /// defined by the same point already exists.
    ///
    /// In the context of of vertex uniqueness, points that are close to each
    /// other are considered identical. The minimum distance between distinct
    /// vertices can be configured using [`Shape::with_minimum_distance`].
    ///
    /// # Implementation note
    ///
    /// This method is intended to actually validate vertex uniqueness: To
    /// panic, if duplicate vertices are found. This is currently not possible,
    /// as the presence of bugs in the sweep and transform code would basically
    /// break ever model, due to validation errors.
    ///
    /// In the future, this method is likely to validate more than just vertex
    /// uniqueness. See documentation of [`crate::kernel`] for some context on
    /// that.
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

    const MIN_DISTANCE: f64 = 5e-7;

    #[test]
    fn create_valid() {
        let mut shape = Shape::new().with_min_distance(MIN_DISTANCE);

        shape.vertices().create(Point::from([0., 0., 0.]));
        shape.vertices().create(Point::from([5e-6, 0., 0.]));
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn create_invalid() {
        let mut shape = Shape::new().with_min_distance(MIN_DISTANCE);

        shape.vertices().create(Point::from([0., 0., 0.]));
        shape.vertices().create(Point::from([5e-8, 0., 0.]));
    }
}
