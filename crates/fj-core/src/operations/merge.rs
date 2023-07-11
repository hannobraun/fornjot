use crate::objects::Solid;

use super::UpdateSolid;

/// Merge two [`Solid`]s
pub trait Merge {
    /// Merge this solid with another
    #[must_use]
    fn merge(&self, other: &Self) -> Self;
}

impl Merge for Solid {
    fn merge(&self, other: &Self) -> Self {
        self.add_shells(other.shells().cloned())
    }
}
