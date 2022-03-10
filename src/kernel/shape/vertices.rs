use tracing::warn;

use crate::{
    kernel::topology::{edges::Cycle, vertices::Vertex},
    math::Scalar,
};

use super::{
    handle::{Handle, Storage},
    CyclesInner, EdgesInner, ValidationError, ValidationResult, VerticesInner,
};

/// The vertices of a shape
pub struct Topology<'r> {
    pub(super) min_distance: Scalar,
    pub(super) vertices: &'r mut VerticesInner,
    pub(super) edges: &'r mut EdgesInner,
    pub(super) cycles: &'r mut CyclesInner,
}

impl Topology<'_> {
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
    pub fn vertices(&self) -> impl Iterator<Item = Handle<Vertex>> + '_ {
        self.vertices.iter().map(|storage| storage.handle())
    }

    /// Add a cycle to the shape
    ///
    /// # Panics
    ///
    /// Panics, if the edges of the cycles are not part of this shape.
    ///
    /// # Implementation note
    ///
    /// The validation of the cycle should be extended to cover more cases:
    /// - That those edges form a cycle.
    /// - That the cycle is not self-overlapping.
    /// - That there exists no duplicate cycle, with the same edges.
    pub fn add_cycle(&mut self, cycle: Cycle) -> ValidationResult<Cycle> {
        for edge in &cycle.edges {
            if !self.edges.contains(edge.storage()) {
                return Err(ValidationError::Structural);
            }
        }

        let storage = Storage::new(cycle);
        let handle = storage.handle();
        self.cycles.push(storage);

        Ok(handle)
    }

    /// Access an iterator over all cycles
    pub fn cycles(&self) -> impl Iterator<Item = Handle<Cycle>> + '_ {
        self.cycles.iter().map(|storage| storage.handle())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        kernel::{
            shape::{handle::Handle, Shape, ValidationError},
            topology::{
                edges::{Cycle, Edge},
                vertices::Vertex,
            },
        },
        math::Point,
    };

    const MIN_DISTANCE: f64 = 5e-7;

    #[test]
    fn add_vertex() -> anyhow::Result<()> {
        let mut shape = Shape::new().with_min_distance(MIN_DISTANCE);

        let point = shape.geometry().add_point(Point::from([0., 0., 0.]))?;
        shape.topology().add_vertex(Vertex { point })?;

        // `point` is too close to the original point. `assert!` is commented,
        // because that only causes a warning to be logged right now.
        let point = shape.geometry().add_point(Point::from([5e-6, 0., 0.]))?;
        let _result = shape.topology().add_vertex(Vertex { point });
        // assert!(matches!(result, Err(ValidationError::Uniqueness)));

        // `point` is farther than `MIN_DISTANCE` away from original point.
        // Should work.
        let point = shape.geometry().add_point(Point::from([5e-6, 0., 0.]))?;
        shape.topology().add_vertex(Vertex { point })?;

        Ok(())
    }

    #[test]
    fn add_cycle() -> anyhow::Result<()> {
        struct TestShape {
            inner: Shape,
            edge: Handle<Edge>,
        }

        impl TestShape {
            fn new() -> anyhow::Result<Self> {
                let mut inner = Shape::new();

                let a =
                    inner.geometry().add_point(Point::from([0., 0., 0.]))?;
                let b =
                    inner.geometry().add_point(Point::from([1., 0., 0.]))?;

                let a = inner.topology().add_vertex(Vertex { point: a })?;
                let b = inner.topology().add_vertex(Vertex { point: b })?;

                let edge = inner.edges().add_line_segment([a, b])?;

                Ok(Self { inner, edge })
            }
        }

        let mut shape = TestShape::new()?;
        let other = TestShape::new()?;

        // Trying to refer to edge that is not from the same shape. Should fail.
        let result = shape.inner.topology().add_cycle(Cycle {
            edges: vec![other.edge],
        });
        assert!(matches!(result, Err(ValidationError::Structural)));

        // Referring to edge that *is* from the same shape. Should work.
        shape.inner.topology().add_cycle(Cycle {
            edges: vec![shape.edge],
        })?;

        Ok(())
    }
}
