use crate::{
    objects::{Curve, GlobalCurve, Objects, Surface},
    path::SurfacePath,
    storage::{Handle, HandleWrapper},
    validate::ValidationError,
};

/// A partial [`Curve`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialCurve {
    path: Option<SurfacePath>,
    surface: Option<Handle<Surface>>,
    global_form: Option<HandleWrapper<GlobalCurve>>,
}

impl PartialCurve {
    /// Access the path that defines the [`Curve`]
    pub fn path(&self) -> Option<SurfacePath> {
        self.path
    }

    /// Access the surface that the [`Curve`] is defined in
    pub fn surface(&self) -> Option<Handle<Surface>> {
        self.surface.clone()
    }

    /// Access the global form of the [`Curve`]
    pub fn global_form(&self) -> Option<Handle<GlobalCurve>> {
        self.global_form
            .clone()
            .map(|handle_wrapper| handle_wrapper.0)
    }

    /// Provide a path for the partial curve
    pub fn with_path(mut self, path: Option<SurfacePath>) -> Self {
        if let Some(path) = path {
            self.path = Some(path);
        }
        self
    }

    /// Provide a surface for the partial curve
    pub fn with_surface(mut self, surface: Option<Handle<Surface>>) -> Self {
        if let Some(surface) = surface {
            self.surface = Some(surface);
        }
        self
    }

    /// Provide a global form for the partial curve
    pub fn with_global_form(
        mut self,
        global_form: Option<impl Into<HandleWrapper<GlobalCurve>>>,
    ) -> Self {
        if let Some(global_form) = global_form {
            self.global_form = Some(global_form.into());
        }
        self
    }

    /// Build a full [`Curve`] from the partial curve
    pub fn build(
        self,
        objects: &Objects,
    ) -> Result<Handle<Curve>, ValidationError> {
        let path = self.path.expect("Can't build `Curve` without path");
        let surface =
            self.surface.expect("Can't build `Curve` without surface");

        let global_form = match self.global_form {
            Some(global_form) => global_form,
            None => objects.global_curves.insert(GlobalCurve)?.into(),
        };

        Ok(objects
            .curves
            .insert(Curve::new(surface, path, global_form))?)
    }
}

impl From<&Curve> for PartialCurve {
    fn from(curve: &Curve) -> Self {
        Self {
            path: Some(curve.path()),
            surface: Some(curve.surface().clone()),
            global_form: Some(curve.global_form().clone().into()),
        }
    }
}

/// A partial [`GlobalCurve`]
///
/// This struct might seem unnecessary. [`GlobalCurve`] literally has nothing in
/// it. Why would we need to represent a part of nothing? However, having this
/// provides some regularity that helps simplify some things within the partial
/// object and builder APIs.
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PartialGlobalCurve;

impl PartialGlobalCurve {
    /// Build a full [`GlobalCurve`] from the partial global curve
    pub fn build(
        self,
        objects: &Objects,
    ) -> Result<Handle<GlobalCurve>, ValidationError> {
        let global_curve = objects.global_curves.insert(GlobalCurve)?;
        Ok(global_curve)
    }
}

impl From<&GlobalCurve> for PartialGlobalCurve {
    fn from(_: &GlobalCurve) -> Self {
        Self
    }
}
