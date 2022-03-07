use crate::kernel::topology::edges::Cycle;

/// The cycles of a shape
pub struct Cycles<'r> {
    pub(super) cycles: &'r mut Vec<Cycle>,
}

impl Cycles<'_> {
    /// Access an iterator over all cycles
    pub fn all(&self) -> impl Iterator<Item = Cycle> + '_ {
        self.cycles.iter().cloned()
    }
}
