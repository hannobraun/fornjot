use fj_math::Point;

use crate::{
    objects::{Curve, GlobalVertex, Objects, Surface, SurfaceVertex, Vertex},
    partial::{MaybePartial, MergeWith},
    partial2::Partial,
    services::Service,
};

/// A partial [`Vertex`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialVertex {
    /// The position of the [`Vertex`]
    pub position: Option<Point<1>>,

    /// The curve that the [`Vertex`] is defined in
    pub curve: Partial<Curve>,

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
        let curve = self.curve.build(objects);

        let surface_form = self
            .surface_form
            .update_partial(|mut partial| {
                let position = partial.position.unwrap_or_else(|| {
                    curve.path().point_from_path_coords(position)
                });

                partial.position = Some(position);

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
            curve: self.curve,
            surface_form: self.surface_form.merge_with(other.surface_form),
        }
    }
}

impl From<&Vertex> for PartialVertex {
    fn from(vertex: &Vertex) -> Self {
        Self {
            position: Some(vertex.position()),
            curve: Partial::from_full_entry_point(vertex.curve().clone()),
            surface_form: vertex.surface_form().clone().into(),
        }
    }
}

impl MaybePartial<Vertex> {
    /// Access the curve
    pub fn curve(&self) -> Partial<Curve> {
        match self {
            Self::Full(full) => {
                Partial::from_full_entry_point(full.curve().clone())
            }
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
    pub surface: Partial<Surface>,

    /// The global form of the [`SurfaceVertex`]
    pub global_form: Partial<GlobalVertex>,
}

impl PartialSurfaceVertex {
    /// Build a full [`SurfaceVertex`] from the partial surface vertex
    pub fn build(mut self, objects: &mut Service<Objects>) -> SurfaceVertex {
        let position = self
            .position
            .expect("Can't build `SurfaceVertex` without position");
        let surface = self.surface.build(objects);

        self.global_form.write().position.get_or_insert_with(|| {
            surface.geometry().point_from_surface_coords(position)
        });
        let global_form = self.global_form.build(objects);

        SurfaceVertex::new(position, surface, global_form)
    }
}

impl MergeWith for PartialSurfaceVertex {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            position: self.position.merge_with(other.position),
            surface: self.surface,
            global_form: self.global_form,
        }
    }
}

impl From<&SurfaceVertex> for PartialSurfaceVertex {
    fn from(surface_vertex: &SurfaceVertex) -> Self {
        Self {
            position: Some(surface_vertex.position()),
            surface: Partial::from_full_entry_point(
                surface_vertex.surface().clone(),
            ),
            global_form: Partial::from_full_entry_point(
                surface_vertex.global_form().clone(),
            ),
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
    pub fn surface(&self) -> Partial<Surface> {
        match self {
            Self::Full(full) => {
                Partial::from_full_entry_point(full.surface().clone())
            }
            Self::Partial(partial) => partial.surface.clone(),
        }
    }

    /// Access the global form
    pub fn global_form(&self) -> Partial<GlobalVertex> {
        match self {
            Self::Full(full) => {
                Partial::from_full_entry_point(full.global_form().clone())
            }
            Self::Partial(partial) => partial.global_form.clone(),
        }
    }
}
