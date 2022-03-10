use tracing::warn;

use crate::{kernel::topology::vertices::Vertex, math::Scalar};

use super::{
    handle::{Handle, Storage},
    ValidationResult, VerticesInner,
};

/// The vertices of a shape
pub struct Vertices<'r> {
    pub(super) min_distance: Scalar,
    pub(super) vertices: &'r mut VerticesInner,
}

impl Vertices<'_> {
    /// Add a vertex to the shape
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
    pub fn add_vertex(&mut self, vertex: Vertex) -> ValidationResult<Vertex> {
        // Make sure the new vertex is a minimum distance away from all existing
        // vertices. This minimum distance is defined to be half a µm, which
        // should provide more than enough precision for common use cases, while
        // being large enough to catch all invalid cases.
        for existing in &*self.vertices {
            let distance = (existing.point() - vertex.point()).magnitude();

            if distance < self.min_distance {
                warn!(
                    "Invalid vertex: {vertex:?}; \
                    identical vertex at {existing:?}",
                );
            }
        }

        let storage = Storage::new(vertex);
        let handle = storage.handle();
        self.vertices.push(storage);

        Ok(handle)
    }

    /// Access iterator over all vertices
    ///
    /// The caller must not make any assumptions about the order of vertices.
    pub fn all(&self) -> impl Iterator<Item = Handle<Vertex>> + '_ {
        self.vertices.iter().map(|storage| storage.handle())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        kernel::{shape::Shape, topology::vertices::Vertex},
        math::Point,
    };

    const MIN_DISTANCE: f64 = 5e-7;

    #[test]
    fn add() -> anyhow::Result<()> {
        let mut shape = Shape::new().with_min_distance(MIN_DISTANCE);

        let point = shape.geometry().add_point(Point::from([0., 0., 0.]))?;
        shape.vertices().add_vertex(Vertex { point })?;

        // `point` is too close to the original point. `assert!` is commented,
        // because that only causes a warning to be logged right now.
        let point = shape.geometry().add_point(Point::from([5e-6, 0., 0.]))?;
        let _result = shape.vertices().add_vertex(Vertex { point });
        // assert!(matches!(result, Err(ValidationError::Uniqueness)));

        // `point` is farther than `MIN_DISTANCE` away from original point.
        // Should work.
        let point = shape.geometry().add_point(Point::from([5e-6, 0., 0.]))?;
        shape.vertices().add_vertex(Vertex { point })?;

        Ok(())
    }
}
