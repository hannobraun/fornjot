use crate::{
    objects::{Curve, GlobalCurve, Objects, Surface},
    partial::{MaybePartial, MergeWith, Mergeable},
    path::SurfacePath,
    storage::Handle,
    validate::ValidationError,
};

/// A partial [`Curve`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialCurve {
    /// The path that defines the [`Curve`]
    pub path: Option<SurfacePath>,

    /// The surface that the [`Curve`] is defined in
    pub surface: Option<Handle<Surface>>,

    /// The global form of the [`Curve`]
    pub global_form: Option<MaybePartial<GlobalCurve>>,
}

impl PartialCurve {
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
        global_form: Option<impl Into<MaybePartial<GlobalCurve>>>,
    ) -> Self {
        if let Some(global_form) = global_form {
            self.global_form = Some(global_form.into());
        }
        self
    }

    /// Build a full [`Curve`] from the partial curve
    pub fn build(self, objects: &Objects) -> Result<Curve, ValidationError> {
        let path = self.path.expect("Can't build `Curve` without path");
        let surface =
            self.surface.expect("Can't build `Curve` without surface");

        let global_form = match self.global_form {
            Some(global_form) => global_form,
            None => objects.global_curves.insert(GlobalCurve)?.into(),
        }
        .into_full(objects)?;

        Ok(Curve::new(surface, path, global_form))
    }
}

impl MergeWith for PartialCurve {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            path: self.path.merge_with(other.path),
            surface: self.surface.merge_with(other.surface),
            global_form: Mergeable(self.global_form)
                .merge_with(Mergeable(other.global_form))
                .0,
        }
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
    pub fn build(self, _: &Objects) -> Result<GlobalCurve, ValidationError> {
        Ok(GlobalCurve)
    }
}

impl MergeWith for PartialGlobalCurve {
    fn merge_with(self, _: impl Into<Self>) -> Self {
        Self
    }
}

impl From<&GlobalCurve> for PartialGlobalCurve {
    fn from(_: &GlobalCurve) -> Self {
        Self
    }
}
