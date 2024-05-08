//! Reverse the direction/orientation of objects

use crate::Core;

mod cycle;
mod face;
mod half_edge;
mod region;

/// Reverse the direction/orientation of an object
pub trait Reverse {
    /// Reverse the direction/orientation of the object
    #[must_use]
    fn reverse(&self, core: &mut Core) -> Self;
}

/// Reverse the direction of the curve coordinate systems within an object
pub trait ReverseCurveCoordinateSystems {
    /// The type of the reversed object
    type Reversed;

    /// Reverse the direction of the curve coordinate systems within an object
    ///
    /// This will not have any effect on object positions in global coordinates.
    #[must_use]
    fn reverse_curve_coordinate_systems(
        self,
        core: &mut Core,
    ) -> Self::Reversed;
}
