use tracing::warn;

use crate::{kernel::topology::vertices::Vertex, math::Scalar};

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
    pub fn create(&mut self, vertex: impl Into<Vertex>) -> Handle<Vertex> {
        let vertex = vertex.into();
        let point = vertex.point();
        let handle = Handle::new(vertex);

        // Make sure the new vertex is a minimum distance away from all existing
        // vertices. This minimum distance is defined to be half a µm, which
        // should provide more than enough precision for common use cases, while
        // being large enough to catch all invalid cases.
        for existing in &*self.vertices {
            let existing = existing.get();

            if (existing.point() - point).magnitude() < self.min_distance {
                warn!(
                    "Invalid vertex: {point:?}; \
                    identical vertex at {existing:?}",
                );
            }
        }

        self.vertices.push(handle.inner());

        handle
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
        // Test is ignored, until vertex validation can be enabled for real.
        // See implementation note on `Vertices::create`.

        let mut shape = Shape::new().with_min_distance(MIN_DISTANCE);

        shape.vertices().create(Point::from([0., 0., 0.]));
        shape.vertices().create(Point::from([5e-8, 0., 0.]));
    }
}
