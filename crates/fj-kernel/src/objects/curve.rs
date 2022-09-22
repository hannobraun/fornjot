use crate::{
    partial::{PartialCurve, PartialGlobalCurve},
    path::{GlobalPath, SurfacePath},
    stores::{Handle, Stores},
};

use super::Surface;

/// A curve, defined in local surface coordinates
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Curve {
    path: SurfacePath,
    surface: Surface,
    global_form: Handle<GlobalCurve>,
}

impl Curve {
    /// Build a `Curve` using [`PartialCurve`]
    pub fn partial(stores: &Stores, surface: Surface) -> PartialCurve {
        PartialCurve { stores, surface }
    }

    /// Construct a new instance of `Curve`
    pub fn new(
        surface: Surface,
        path: SurfacePath,
        global_form: Handle<GlobalCurve>,
    ) -> Self {
        Self {
            surface,
            path,
            global_form,
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
    pub fn global_form(&self) -> &Handle<GlobalCurve> {
        &self.global_form
    }
}

/// A curve, defined in global (3D) coordinates
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalCurve {
    path: GlobalPath,
}

impl GlobalCurve {
    /// Create a [`PartialGlobalCurve`]
    ///
    /// This function exists just for convenience, and will just return a
    /// default [`PartialGlobalCurve`].
    pub fn partial() -> PartialGlobalCurve {
        PartialGlobalCurve::default()
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
