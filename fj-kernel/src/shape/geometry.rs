use fj_math::{Point, Transform};

use crate::{
    geometry::{Curve, Surface},
    topology::Face,
};

use super::{
    stores::{Curves, Faces, Points, Surfaces},
    Iter,
};

/// API to access a shape's geometry
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
    /// Transform the geometry of the shape
    ///
    /// Since the topological types refer to geometry, and don't contain any
    /// geometry themselves, this transforms the whole shape.
    pub fn transform(&mut self, transform: &Transform) {
        self.points
            .update(|point| *point = transform.transform_point(point));
        self.curves
            .update(|curve| *curve = curve.transform(transform));
        self.surfaces
            .update(|surface| *surface = surface.transform(transform));

        // While some faces use triangle representation, we need this weird
        // workaround here.
        self.faces.update(|mut face| {
            use std::ops::DerefMut as _;
            if let Face::Triangles(triangles) = face.deref_mut() {
                for triangle in triangles {
                    *triangle = transform.transform_triangle(triangle);
                }
            }
        });
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
