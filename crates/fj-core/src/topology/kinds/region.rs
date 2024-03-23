//! A single, continues 2d region

use crate::{
    storage::Handle,
    topology::{Cycle, ObjectSet},
};

/// A single, continuous 2d region, may contain holes
///
/// Interior cycles must have the opposite winding of the exterior cycle,
/// meaning on the front side of the region, they must appear clockwise. This
/// means that all [`HalfEdge`]s that bound a `Region` have the interior of the
/// region on their left side (on the region's front side).
///
/// [`HalfEdge`]: crate::topology::HalfEdge
#[derive(Clone, Debug)]
pub struct Region {
    exterior: Handle<Cycle>,
    interiors: ObjectSet<Cycle>,
}

impl Region {
    /// Construct an instance of `Region`
    pub fn new(
        exterior: Handle<Cycle>,
        interiors: impl IntoIterator<Item = Handle<Cycle>>,
    ) -> Self {
        Self {
            exterior,
            interiors: interiors.into_iter().collect(),
        }
    }

    /// Access the cycle that bounds the region on the outside
    pub fn exterior(&self) -> &Handle<Cycle> {
        &self.exterior
    }

    /// Access the cycles that bound the region on the inside
    ///
    /// Each of these cycles defines a hole in the region .
    pub fn interiors(&self) -> &ObjectSet<Cycle> {
        &self.interiors
    }

    /// Access all cycles of the region (both exterior and interior)
    pub fn all_cycles(&self) -> impl Iterator<Item = &Handle<Cycle>> {
        // It would be nice to return `&ObjectSet` here, but I don't see a way
        // for doing that here *and* in `interiors`.
        [self.exterior()].into_iter().chain(self.interiors())
    }
}
