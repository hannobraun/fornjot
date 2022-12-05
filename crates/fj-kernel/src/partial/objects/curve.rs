use crate::{
    geometry::path::SurfacePath,
    objects::{Curve, GlobalCurve, Objects, Surface},
    partial::{MaybePartial, MergeWith, Replace},
    partial2::Partial,
    services::Service,
    storage::Handle,
};

/// A partial [`Curve`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialCurve {
    /// The path that defines the [`Curve`]
    pub path: Option<SurfacePath>,

    /// The surface that the [`Curve`] is defined in
    pub surface: Partial<Surface>,

    /// The global form of the [`Curve`]
    pub global_form: Partial<GlobalCurve>,
}

impl PartialCurve {
    /// Build a full [`Curve`] from the partial curve
    pub fn build(self, objects: &mut Service<Objects>) -> Curve {
        let path = self.path.expect("Can't build `Curve` without path");
        let surface = self.surface.build(objects);

        let global_form = self.global_form.build(objects);

        Curve::new(surface, path, global_form)
    }
}

impl MergeWith for PartialCurve {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            path: self.path.merge_with(other.path),
            surface: self.surface,
            global_form: self.global_form,
        }
    }
}

impl Replace<Surface> for PartialCurve {
    fn replace(&mut self, surface: Handle<Surface>) -> &mut Self {
        self.surface = Partial::from_full_entry_point(surface);
        self
    }
}

impl From<&Curve> for PartialCurve {
    fn from(curve: &Curve) -> Self {
        Self {
            path: Some(curve.path()),
            surface: Partial::from_full_entry_point(curve.surface().clone()),
            global_form: Partial::from_full_entry_point(
                curve.global_form().clone(),
            ),
        }
    }
}

impl MaybePartial<Curve> {
    /// Access the path
    pub fn path(&self) -> Option<SurfacePath> {
        match self {
            Self::Full(full) => Some(full.path()),
            Self::Partial(partial) => partial.path,
        }
    }

    /// Access the surface
    pub fn surface(&self) -> Partial<Surface> {
        match self {
            Self::Full(full) => {
                Partial::from_full_entry_point(full.surface().clone())
            }
            Self::Partial(partial) => partial.surface.clone(),
        }
    }

    /// Access the global form
    pub fn global_form(&self) -> Partial<GlobalCurve> {
        match self {
            Self::Full(full) => {
                Partial::from_full_entry_point(full.global_form().clone())
            }
            Self::Partial(partial) => partial.global_form.clone(),
        }
    }
}
