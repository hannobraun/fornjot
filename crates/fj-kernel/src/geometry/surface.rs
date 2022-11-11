//! The geometry that defines a surface

use fj_math::Vector;

use super::path::GlobalPath;

/// The geometry that defines a surface
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct SurfaceGeometry {
    /// The u-axis of the surface
    pub u: GlobalPath,

    /// The v-axis of the surface
    pub v: Vector<3>,
}
