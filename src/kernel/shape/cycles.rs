use crate::kernel::topology::edges::Cycle;

use super::{
    handle::{Handle, Storage},
    CyclesInner, EdgesInner,
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
    pub fn add(&mut self, cycle: Cycle) -> Handle<Cycle> {
        for edge in &cycle.edges {
            assert!(
                self.edges.contains(edge.storage()),
                "Cycle validation failed: {edge:?} is not part of the shape",
            );
        }

        let storage = Storage::new(cycle);
        let handle = storage.handle();
        self.cycles.push(storage);

        handle
    }

    /// Access an iterator over all cycles
    pub fn all(&self) -> impl Iterator<Item = Handle<Cycle>> + '_ {
        self.cycles.iter().map(|storage| storage.handle())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        kernel::{shape::Shape, topology::edges::Cycle},
        math::Point,
    };

    #[test]
    fn add_valid() {
        let mut shape = Shape::new();

        let a = shape.vertices().add(Point::from([0., 0., 0.]));
        let b = shape.vertices().add(Point::from([1., 0., 0.]));

        let edge = shape.edges().add_line_segment([a, b]);

        shape.cycles().add(Cycle { edges: vec![edge] });
    }

    #[test]
    #[should_panic]
    fn add_invalid() {
        let mut shape = Shape::new();
        let mut other = Shape::new();

        let a = other.vertices().add(Point::from([0., 0., 0.]));
        let b = other.vertices().add(Point::from([1., 0., 0.]));

        let edge = other.edges().add_line_segment([a, b]);

        shape.cycles().add(Cycle { edges: vec![edge] });
    }
}
