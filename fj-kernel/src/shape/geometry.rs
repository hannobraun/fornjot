use fj_math::{Point, Transform};

use crate::{
    geometry::{Curve, Surface},
    topology::Face,
};

use super::{
    handle::Handle,
    stores::{Curves, Faces, Points, Surfaces},
    Iter,
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
        self.points.add(point)
    }

    /// Add a curve to the shape
    pub fn add_curve(&mut self, curve: Curve) -> Handle<Curve> {
        self.curves.add(curve)
    }

    /// Add a surface to the shape
    pub fn add_surface(&mut self, surface: Surface) -> Handle<Surface> {
        self.surfaces.add(surface)
    }

    /// Transform the geometry of the shape
    ///
    /// Since the topological types refer to geometry, and don't contain any
    /// geometry themselves, this transforms the whole shape.
    pub fn transform(&mut self, transform: &Transform) {
        for mut point in self.points.iter_mut() {
            *point = transform.transform_point(&point);
        }
        for mut curve in self.curves.iter_mut() {
            *curve = curve.transform(transform);
        }
        for mut surface in self.surfaces.iter_mut() {
            *surface = surface.transform(transform);
        }

        // While some faces use triangle representation, we need this weird
        // workaround here.
        for mut face in self.faces.iter_mut() {
            use std::ops::DerefMut as _;
            if let Face::Triangles(triangles) = face.deref_mut() {
                for triangle in triangles {
                    *triangle = transform.transform_triangle(triangle);
                }
            }
        }
    }

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
