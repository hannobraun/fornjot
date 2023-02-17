use crate::{
    geometry::path::SurfacePath,
    storage::{Handle, HandleWrapper},
};

/// A curve, defined in local surface coordinates
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Curve {
    path: SurfacePath,
    global_form: HandleWrapper<GlobalCurve>,
}

impl Curve {
    /// Construct a new instance of `Curve`
    pub fn new(
        path: SurfacePath,
        global_form: impl Into<HandleWrapper<GlobalCurve>>,
    ) -> Self {
        Self {
            path,
            global_form: global_form.into(),
        }
    }

    /// Access the path that defines the curve
    pub fn path(&self) -> SurfacePath {
        self.path
    }

    /// Access the global form of the curve
    pub fn global_form(&self) -> &Handle<GlobalCurve> {
        &self.global_form
    }
}

/// A curve, defined in global (3D) coordinates
#[derive(Clone, Copy, Debug)]
pub struct GlobalCurve;
