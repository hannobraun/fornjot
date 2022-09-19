use crate::{
    path::SurfacePath,
    stores::{Handle, HandleWrapper, Stores},
};

use super::Surface;

/// A curve, defined in local surface coordinates
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Curve {
    path: SurfacePath,
    surface: Surface,
    global_form: HandleWrapper<GlobalCurve>,
}

impl Curve {
    /// Construct a new instance of `Curve`
    pub fn new(
        surface: Surface,
        path: SurfacePath,
        global_form: impl Into<HandleWrapper<GlobalCurve>>,
    ) -> Self {
        let global_form = global_form.into();

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
#[derive(Clone, Copy, Debug)]
pub struct GlobalCurve;

impl GlobalCurve {
    /// Construct a new instance of `Handle` and add it to the store
    pub fn new(stores: &Stores) -> Handle<Self> {
        stores.global_curves.insert(GlobalCurve)
    }
}
