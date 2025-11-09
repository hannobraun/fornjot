use crate::{
    math::{Aabb, Point},
    topology::{LocalCurve, Surface},
};

/// # An ongoing approximation
pub struct Approx {
    // `Approx` would contain various caches with already computed
    // approximations. Also for performance reasons, but more importantly, to
    // avert the need to recompute already computed approximations from
    // different local representations.
    //
    // This would cause mismatching results due to numerical inaccuracy, leading
    // to problems like open or overlapping triangle meshes. This must be
    // avoided.
}

impl Approx {
    /// # Approximate a curve
    ///
    /// Since a curve is potentially infinite, and different parts of it might
    /// be required at different points in time, this expands any approximation
    /// that might already exist to include the provided bounds, then returns a
    /// reference to the updated polyline.
    pub fn approx_curve(&mut self, _: &LocalCurve, _: &Aabb<1>) -> &Polyline {
        todo!()
    }

    /// # Approximate a surface
    ///
    /// Since a surface is potentially infinite, and different parts of it might
    /// be required at different points in time, this expands any approximation
    /// that might already exist to include the provided bounds, then returns a
    /// reference to the updated triangle mesh.
    pub fn approx_surface(&mut self, _: &Surface, _: &Aabb<2>) -> &TriMesh {
        todo!()
    }

    // Not all methods that we are going to need here have already been written
    // out.
}

pub struct Polyline {
    pub points: ApproxPoint<1>,
}

pub struct TriMesh {
    pub triangles: ApproxPoint<2>,
}

pub struct ApproxPoint<const D: usize> {
    pub local: Point<D>,
    pub global: Point<3>,
}
