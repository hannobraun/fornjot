use fj_math::Point;

use crate::geometry::{Curve, Surface};

use super::{
    stores::{Curves, Points, Surfaces},
    Iter,
};

/// API to access a shape's geometry
pub struct Geometry<'r> {
    pub(super) points: &'r mut Points,
    pub(super) curves: &'r mut Curves,
    pub(super) surfaces: &'r mut Surfaces,
}

impl Geometry<'_> {
    /// Access an iterator over all points
    ///
    /// The caller must not make any assumptions about the order of points.
    pub fn points(&self) -> Iter<Point<3>> {
        self.points.iter()
    }

    /// Access an iterator over all curves
    ///
    /// The caller must not make any assumptions about the order of curves.
    pub fn curves(&self) -> Iter<Curve> {
        self.curves.iter()
    }

    /// Access an iterator over all surfaces
    ///
    /// The caller must not make any assumptions about the order of surfaces.
    pub fn surfaces(&self) -> Iter<Surface> {
        self.surfaces.iter()
    }
}
