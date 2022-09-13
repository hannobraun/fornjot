use crate::{
    builder::{CurveBuilder, GlobalCurveBuilder},
    path::{GlobalPath, SurfacePath},
};

use super::Surface;

/// A curve, defined in local surface coordinates
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Curve {
    path: SurfacePath,
    surface: Surface,
    global: GlobalCurve,
}

impl Curve {
    /// Build a curve using [`CurveBuilder`]
    pub fn build(surface: Surface) -> CurveBuilder {
        CurveBuilder::new(surface)
    }

    /// Construct a new instance of `Curve`
    pub fn new(
        surface: Surface,
        path: SurfacePath,
        global: GlobalCurve,
    ) -> Self {
        Self {
            surface,
            path,
            global,
        }
    }

    /// Access the path that defines this curve
    pub fn path(&self) -> SurfacePath {
        self.path
    }

    /// Access the surface that this curve is defined in
    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    /// Access the global form of this curve
    pub fn global_form(&self) -> &GlobalCurve {
        &self.global
    }
}

/// A curve, defined in global (3D) coordinates
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalCurve {
    path: GlobalPath,
}

impl GlobalCurve {
    /// Build a curve using [`GlobalCurveBuilder`]
    pub fn build() -> GlobalCurveBuilder {
        GlobalCurveBuilder
    }

    /// Construct a `GlobalCurve` from the path that defines it
    pub fn from_path(path: GlobalPath) -> Self {
        Self { path }
    }

    /// Access the path that defines this curve
    pub fn path(&self) -> GlobalPath {
        self.path
    }
}
