use crate::{
    geometry::path::SurfacePath,
    objects::Surface,
    storage::{Handle, HandleWrapper},
};

/// A curve, defined in local surface coordinates
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Curve {
    path: SurfacePath,
    surface: Handle<Surface>,
    global_form: HandleWrapper<GlobalCurve>,
}

impl Curve {
    /// Construct a new instance of `Curve`
    pub fn new(
        surface: Handle<Surface>,
        path: SurfacePath,
        global_form: impl Into<HandleWrapper<GlobalCurve>>,
    ) -> Self {
        Self {
            surface,
            path,
            global_form: global_form.into(),
        }
    }

    /// Access the path that defines the curve
    pub fn path(&self) -> SurfacePath {
        self.path
    }

    /// Access the surface that the curve is defined in
    pub fn surface(&self) -> &Handle<Surface> {
        &self.surface
    }

    /// Access the global form of the curve
    pub fn global_form(&self) -> &Handle<GlobalCurve> {
        &self.global_form
    }
}

/// A curve, defined in global (3D) coordinates
#[derive(Clone, Copy, Debug)]
pub struct GlobalCurve;
