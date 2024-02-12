//! # Operations to merge objects
//!
//! See [`Merge`], which is currently the only trait in this module, for more
//! information.

use crate::{objects::Solid, Instance};

use super::update::UpdateSolid;

/// Merge two [`Solid`]s
pub trait Merge {
    /// Merge this solid with another
    #[must_use]
    fn merge(&self, other: &Self, core: &mut Instance) -> Self;
}

impl Merge for Solid {
    fn merge(&self, other: &Self, _: &mut Instance) -> Self {
        self.add_shells(other.shells().iter().cloned())
    }
}
