use fj_math::Point;

use crate::{
    builder::GlobalVertexBuilder,
    objects::{Curve, GlobalVertex, Objects, Surface, SurfaceVertex, Vertex},
    partial::{MaybePartial, MergeWith},
    services::Service,
    storage::Handle,
};

/// A partial [`Vertex`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialVertex {
    /// The position of the [`Vertex`]
    pub position: Option<Point<1>>,

    /// The curve that the [`Vertex`] is defined in
    pub curve: MaybePartial<Curve>,

    /// The surface form of the [`Vertex`]
    pub surface_form: MaybePartial<SurfaceVertex>,
}

impl PartialVertex {
    /// Build a full [`Vertex`] from the partial vertex
    ///
    /// # Panics
    ///
    /// Panics, if position has not been provided.
    ///
    /// Panics, if curve has not been provided.
    pub fn build(self, objects: &mut Service<Objects>) -> Vertex {
        let position = self
            .position
            .expect("Cant' build `Vertex` without position");
        let curve = self.curve.into_full(objects);

        let surface_form = self
            .surface_form
            .update_partial(|mut partial| {
                let position = partial.position.unwrap_or_else(|| {
                    curve.path().point_from_path_coords(position)
                });

                partial.position = Some(position);
                partial.surface = Some(curve.surface().clone());

                partial
            })
            .into_full(objects);

        Vertex::new(position, curve, surface_form)
    }
}

impl MergeWith for PartialVertex {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            position: self.position.merge_with(other.position),
            curve: self.curve.merge_with(other.curve),
            surface_form: self.surface_form.merge_with(other.surface_form),
        }
    }
}

impl From<&Vertex> for PartialVertex {
    fn from(vertex: &Vertex) -> Self {
        Self {
            position: Some(vertex.position()),
            curve: vertex.curve().clone().into(),
            surface_form: vertex.surface_form().clone().into(),
        }
    }
}

impl MaybePartial<Vertex> {
    /// Access the curve
    pub fn curve(&self) -> MaybePartial<Curve> {
        match self {
            Self::Full(full) => full.curve().clone().into(),
            Self::Partial(partial) => partial.curve.clone(),
        }
    }

    /// Access the surface form
    pub fn surface_form(&self) -> MaybePartial<SurfaceVertex> {
        match self {
            Self::Full(full) => full.surface_form().clone().into(),
            Self::Partial(partial) => partial.surface_form.clone(),
        }
    }
}

/// A partial [`SurfaceVertex`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialSurfaceVertex {
    /// The position of the [`SurfaceVertex`]
    pub position: Option<Point<2>>,

    /// The surface that the [`SurfaceVertex`] is defined in
    pub surface: Option<Handle<Surface>>,

    /// The global form of the [`SurfaceVertex`]
    pub global_form: MaybePartial<GlobalVertex>,
}

impl PartialSurfaceVertex {
    /// Build a full [`SurfaceVertex`] from the partial surface vertex
    pub fn build(mut self, objects: &mut Service<Objects>) -> SurfaceVertex {
        let position = self
            .position
            .expect("Can't build `SurfaceVertex` without position");
        let surface = self
            .surface
            .expect("Can't build `SurfaceVertex` without `Surface`");

        if self.global_form.position().is_none() {
            self.global_form = self.global_form.merge_with(
                PartialGlobalVertex::from_surface_and_position(
                    &surface.geometry(),
                    position,
                ),
            );
        }
        let global_form = self.global_form.into_full(objects);

        SurfaceVertex::new(position, surface, global_form)
    }
}

impl MergeWith for PartialSurfaceVertex {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            position: self.position.merge_with(other.position),
            surface: self.surface.merge_with(other.surface),
            global_form: self.global_form.merge_with(other.global_form),
        }
    }
}

impl From<&SurfaceVertex> for PartialSurfaceVertex {
    fn from(surface_vertex: &SurfaceVertex) -> Self {
        Self {
            position: Some(surface_vertex.position()),
            surface: Some(surface_vertex.surface().clone()),
            global_form: (surface_vertex.global_form().clone()).into(),
        }
    }
}

impl MaybePartial<SurfaceVertex> {
    /// Access the position
    pub fn position(&self) -> Option<Point<2>> {
        match self {
            Self::Full(full) => Some(full.position()),
            Self::Partial(partial) => partial.position,
        }
    }

    /// Access the surface
    pub fn surface(&self) -> Option<Handle<Surface>> {
        match self {
            Self::Full(full) => Some(full.surface().clone()),
            Self::Partial(partial) => partial.surface.clone(),
        }
    }

    /// Access the global form
    pub fn global_form(&self) -> MaybePartial<GlobalVertex> {
        match self {
            Self::Full(full) => full.global_form().clone().into(),
            Self::Partial(partial) => partial.global_form.clone(),
        }
    }
}

/// A partial [`GlobalVertex`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PartialGlobalVertex {
    /// The position of the [`GlobalVertex`]
    pub position: Option<Point<3>>,
}

impl PartialGlobalVertex {
    /// Build a full [`GlobalVertex`] from the partial global vertex
    pub fn build(self, _: &Objects) -> GlobalVertex {
        let position = self
            .position
            .expect("Can't build a `GlobalVertex` without a position");

        GlobalVertex::new(position)
    }
}

impl MergeWith for PartialGlobalVertex {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            position: self.position.merge_with(other.position),
        }
    }
}

impl From<&GlobalVertex> for PartialGlobalVertex {
    fn from(global_vertex: &GlobalVertex) -> Self {
        Self {
            position: Some(global_vertex.position()),
        }
    }
}

impl MaybePartial<GlobalVertex> {
    /// Access the position
    pub fn position(&self) -> Option<Point<3>> {
        match self {
            Self::Full(full) => Some(full.position()),
            Self::Partial(partial) => partial.position,
        }
    }
}
