use crate::{
    kernel::geometry::{Curve, Surface},
    math::Point,
};

use super::handle::{Handle, Storage};

/// API to access a shape's geometry
///
/// Other than topology, geometry doesn't need to be validated. Hence adding
/// geometry is infallible.
///
/// There are several reasons for this:
/// - Geometry doesn't refer to other objects, so structural validation doesn't
///   apply.
/// - There simply no reason that geometry needs to be unique. In addition, it's
///   probably quite hard to rule out generating duplicate geometry. Think about
///   line segment edges that are on identical lines, but are created
///   separately.
/// - Geometric validation doesn't apply either. It simply doesn't matter, if
///   curves or surfaces intersect, for example, as long as they don't do that
///   where an edge or face is defined.
pub struct Geometry;

impl Geometry {
    /// Add a point to the shape
    pub fn add_point(&mut self, point: Point<3>) -> Handle<Point<3>> {
        Storage::new(point).handle()
    }

    /// Add a curve to the shape
    pub fn add_curve(&mut self, curve: Curve) -> Handle<Curve> {
        Storage::new(curve).handle()
    }

    /// Add a surface to the shape
    pub fn add_surface(&mut self, surface: Surface) -> Handle<Surface> {
        Storage::new(surface).handle()
    }
}
