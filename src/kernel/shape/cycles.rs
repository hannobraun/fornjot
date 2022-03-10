use crate::kernel::topology::edges::Cycle;

use super::{
    handle::{Handle, Storage},
    CyclesInner, EdgesInner, ValidationError, ValidationResult,
};

/// The cycles of a shape
pub struct Cycles<'r> {
    pub(super) edges: &'r mut EdgesInner,
    pub(super) cycles: &'r mut CyclesInner,
}

impl Cycles<'_> {
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
    pub fn add(&mut self, cycle: Cycle) -> ValidationResult<Cycle> {
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
    pub fn all(&self) -> impl Iterator<Item = Handle<Cycle>> + '_ {
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

    #[test]
    fn add() -> anyhow::Result<()> {
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

                let a = inner.vertices().add(Vertex { point: a })?;
                let b = inner.vertices().add(Vertex { point: b })?;

                let edge = inner.edges().add_line_segment([a, b])?;

                Ok(Self { inner, edge })
            }
        }

        let mut shape = TestShape::new()?;
        let other = TestShape::new()?;

        // Trying to refer to edge that is not from the same shape. Should fail.
        let result = shape.inner.cycles().add(Cycle {
            edges: vec![other.edge],
        });
        assert!(matches!(result, Err(ValidationError::Structural)));

        // Referring to edge that *is* from the same shape. Should work.
        shape.inner.cycles().add(Cycle {
            edges: vec![shape.edge],
        })?;

        Ok(())
    }
}
