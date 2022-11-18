use crate::{
    geometry::path::SurfacePath,
    get::Get,
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

    /// Access the path that defines this curve
    pub fn path(&self) -> SurfacePath {
        self.path
    }

    /// Access the surface that this curve is defined in
    pub fn surface(&self) -> &Handle<Surface> {
        &self.surface
    }

    /// Access the global form of this curve
    pub fn global_form(&self) -> &Handle<GlobalCurve> {
        &self.global_form
    }
}

impl Get<Surface> for Curve {
    fn get(&self) -> Handle<Surface> {
        self.surface().clone()
    }
}

impl Get<GlobalCurve> for Curve {
    fn get(&self) -> Handle<GlobalCurve> {
        self.global_form().clone()
    }
}

/// A curve, defined in global (3D) coordinates
#[derive(Clone, Copy, Debug)]
pub struct GlobalCurve;
