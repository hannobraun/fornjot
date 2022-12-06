use fj_math::Point;

use crate::{
    objects::{Curve, Objects, SurfaceVertex, Vertex},
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
    pub surface_form: Partial<SurfaceVertex>,
}

impl PartialVertex {
    /// Build a full [`Vertex`] from the partial vertex
    ///
    /// # Panics
    ///
    /// Panics, if position has not been provided.
    ///
    /// Panics, if curve has not been provided.
    pub fn build(mut self, objects: &mut Service<Objects>) -> Vertex {
        let position = self
            .position
            .expect("Cant' build `Vertex` without position");
        let curve = self.curve.build(objects);

        if self.surface_form.read().position.is_none() {
            self.surface_form.write().position =
                Some(curve.path().point_from_path_coords(position));
        }

        let surface_form = self.surface_form.build(objects);

        Vertex::new(position, curve, surface_form)
    }
}

impl MergeWith for PartialVertex {
    fn merge_with(self, other: impl Into<Self>) -> Self {
        let other = other.into();

        Self {
            position: self.position.merge_with(other.position),
            curve: self.curve,
            surface_form: self.surface_form,
        }
    }
}

impl From<&Vertex> for PartialVertex {
    fn from(vertex: &Vertex) -> Self {
        Self {
            position: Some(vertex.position()),
            curve: Partial::from_full_entry_point(vertex.curve().clone()),
            surface_form: Partial::from_full_entry_point(
                vertex.surface_form().clone(),
            ),
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
    pub fn surface_form(&self) -> Partial<SurfaceVertex> {
        match self {
            Self::Full(full) => {
                Partial::from_full_entry_point(full.surface_form().clone())
            }
            Self::Partial(partial) => partial.surface_form.clone(),
        }
    }
}
