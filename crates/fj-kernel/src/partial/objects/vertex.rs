use fj_math::Point;

use crate::{
    builder::GlobalVertexBuilder,
    objects::{Curve, GlobalVertex, Objects, Surface, SurfaceVertex, Vertex},
    partial::{MaybePartial, MergeWith},
    storage::Handle,
    validate::ValidationError,
};

/// A partial [`Vertex`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default)]
pub struct PartialVertex {
    position: Option<Point<1>>,
    curve: MaybePartial<Curve>,
    surface_form: MaybePartial<SurfaceVertex>,
}

impl PartialVertex {
    /// Access the position of the [`Vertex`] on the curve
    pub fn position(&self) -> Option<Point<1>> {
        self.position
    }

    /// Access the curve that the [`Vertex`] is defined in
    pub fn curve(&self) -> MaybePartial<Curve> {
        self.curve.clone()
    }

    /// Access the surface form of the [`Vertex`]
    pub fn surface_form(&self) -> MaybePartial<SurfaceVertex> {
        self.surface_form.clone()
    }

    /// Provide a position for the partial vertex
    pub fn with_position(
        mut self,
        position: Option<impl Into<Point<1>>>,
    ) -> Self {
        if let Some(position) = position {
            self.position = Some(position.into());
        }
        self
    }

    /// Provide a curve for the partial vertex
    pub fn with_curve(mut self, curve: impl Into<MaybePartial<Curve>>) -> Self {
        self.curve = curve.into();
        self
    }

    /// Provide a surface form for the partial vertex
    pub fn with_surface_form(
        mut self,
        surface_form: impl Into<MaybePartial<SurfaceVertex>>,
    ) -> Self {
        self.surface_form = surface_form.into();
        self
    }

    /// Build a full [`Vertex`] from the partial vertex
    ///
    /// # Panics
    ///
    /// Panics, if position has not been provided.
    ///
    /// Panics, if curve has not been provided.
    pub fn build(self, objects: &Objects) -> Result<Vertex, ValidationError> {
        let position = self
            .position
            .expect("Cant' build `Vertex` without position");
        let curve = self.curve.into_full(objects)?;

        let surface_form = self
            .surface_form
            .update_partial(|partial| {
                let position = partial.position.unwrap_or_else(|| {
                    curve.path().point_from_path_coords(position)
                });

                partial
                    .with_position(Some(position))
                    .with_surface(Some(curve.surface().clone()))
            })
            .into_full(objects)?;

        Ok(Vertex::new(position, curve, surface_form))
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

/// A partial [`SurfaceVertex`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PartialSurfaceVertex {
    position: Option<Point<2>>,
    surface: Option<Handle<Surface>>,
    global_form: MaybePartial<GlobalVertex>,
}

impl PartialSurfaceVertex {
    /// Access the position of the [`SurfaceVertex`]
    pub fn position(&self) -> Option<Point<2>> {
        self.position
    }

    /// Access the surface that the [`SurfaceVertex`] is defined in
    pub fn surface(&self) -> Option<Handle<Surface>> {
        self.surface.clone()
    }

    /// Access the global form of the [`SurfaceVertex`]
    pub fn global_form(&self) -> MaybePartial<GlobalVertex> {
        self.global_form.clone()
    }

    /// Provide a position for the partial surface vertex
    pub fn with_position(
        mut self,
        position: Option<impl Into<Point<2>>>,
    ) -> Self {
        if let Some(position) = position {
            self.position = Some(position.into());
        }
        self
    }

    /// Provide a surface for the partial surface vertex
    pub fn with_surface(mut self, surface: Option<Handle<Surface>>) -> Self {
        if let Some(surface) = surface {
            self.surface = Some(surface);
        }
        self
    }

    /// Provide a global form for the partial surface vertex
    pub fn with_global_form(
        mut self,
        global_form: Option<impl Into<MaybePartial<GlobalVertex>>>,
    ) -> Self {
        if let Some(global_form) = global_form {
            self.global_form = global_form.into();
        }
        self
    }

    /// Build a full [`SurfaceVertex`] from the partial surface vertex
    pub fn build(
        self,
        objects: &Objects,
    ) -> Result<SurfaceVertex, ValidationError> {
        let position = self
            .position
            .expect("Can't build `SurfaceVertex` without position");
        let surface = self
            .surface
            .expect("Can't build `SurfaceVertex` without `Surface`");

        let global_form = self
            .global_form
            .update_partial(|global_form| {
                global_form.update_from_surface_and_position(&surface, position)
            })
            .into_full(objects)?;

        Ok(SurfaceVertex::new(position, surface, global_form))
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

/// A partial [`GlobalVertex`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PartialGlobalVertex {
    /// The position of the [`GlobalVertex`]
    pub position: Option<Point<3>>,
}

impl PartialGlobalVertex {
    /// Provide a position for the partial global vertex
    pub fn with_position(
        mut self,
        position: Option<impl Into<Point<3>>>,
    ) -> Self {
        if let Some(position) = position {
            self.position = Some(position.into());
        }
        self
    }

    /// Build a full [`GlobalVertex`] from the partial global vertex
    pub fn build(self, _: &Objects) -> Result<GlobalVertex, ValidationError> {
        let position = self
            .position
            .expect("Can't build a `GlobalVertex` without a position");

        Ok(GlobalVertex::from_position(position))
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
