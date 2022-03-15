use crate::{
    kernel::geometry::{Curve, Surface},
    math::Point,
};

use super::{
    handle::{Handle, Storage},
    Curves, Faces, Iter, Points, Surfaces,
};

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
pub struct Geometry<'r> {
    pub(super) points: &'r mut Points,
    pub(super) curves: &'r mut Curves,
    pub(super) surfaces: &'r mut Surfaces,

    // This is needed here for a weird workaround, which in turn is necessary
    // because triangle representation still exists. Once triangle
    // representation is no longer a thing, this field can be moved to
    // `Topology`, where it belongs.
    //
    // This issue has some context on triangle representation:
    // https://github.com/hannobraun/Fornjot/issues/97
    pub(super) faces: &'r mut Faces,
}

impl Geometry<'_> {
    /// Add a point to the shape
    pub fn add_point(&mut self, point: Point<3>) -> Handle<Point<3>> {
        let storage = Storage::new(point);
        let handle = storage.handle();

        self.points.push(storage);

        handle
    }

    /// Add a curve to the shape
    pub fn add_curve(&mut self, curve: Curve) -> Handle<Curve> {
        let storage = Storage::new(curve);
        let handle = storage.handle();

        self.curves.push(storage);

        handle
    }

    /// Add a surface to the shape
    pub fn add_surface(&mut self, surface: Surface) -> Handle<Surface> {
        let storage = Storage::new(surface);
        let handle = storage.handle();

        self.surfaces.push(storage);

        handle
    }

    /// Access an iterator over all points
    ///
    /// The caller must not make any assumptions about the order of points.
    pub fn points(&self) -> Iter<Point<3>> {
        Iter::new(self.points)
    }

    /// Access an iterator over all curves
    ///
    /// The caller must not make any assumptions about the order of curves.
    pub fn curves(&self) -> Iter<Curve> {
        Iter::new(self.curves)
    }

    /// Access an iterator over all surfaces
    ///
    /// The caller must not make any assumptions about the order of surfaces.
    pub fn surfaces(&self) -> Iter<Surface> {
        Iter::new(self.surfaces)
    }
}
