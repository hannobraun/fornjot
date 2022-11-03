use fj_math::Point;

use crate::{
    objects::{Curve, GlobalVertex, Objects, Surface, SurfaceVertex, Vertex},
    partial::{HasPartial, MaybePartial},
    storage::Handle,
    validate::ValidationError,
};

/// A partial [`Vertex`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PartialVertex {
    /// The position of the [`Vertex`] on the [`Curve`]
    ///
    /// Must be provided before [`PartialVertex::build`] is called.
    pub position: Option<Point<1>>,

    /// The curve that the [`Vertex`] is defined in
    ///
    /// Must be provided before [`PartialVertex::build`] is called.
    pub curve: MaybePartial<Curve>,

    /// The surface form of the [`Vertex`]
    ///
    /// Can be provided, if already available, or computed from the position on
    /// the [`Curve`].
    pub surface_form: MaybePartial<SurfaceVertex>,
}

impl PartialVertex {
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
    pub fn with_curve(
        mut self,
        curve: Option<impl Into<MaybePartial<Curve>>>,
    ) -> Self {
        if let Some(curve) = curve {
            self.curve = curve.into();
        }
        self
    }

    /// Provide a surface form for the partial vertex
    pub fn with_surface_form(
        mut self,
        surface_form: Option<impl Into<MaybePartial<SurfaceVertex>>>,
    ) -> Self {
        if let Some(surface_form) = surface_form {
            self.surface_form = surface_form.into();
        }
        self
    }

    /// Remove the surface form of the partial vertex, inferring it on build
    pub fn infer_surface_form(mut self) -> Self {
        self.surface_form = SurfaceVertex::partial().into();
        self
    }

    /// Build a full [`Vertex`] from the partial vertex
    ///
    /// # Panics
    ///
    /// Panics, if no position has been provided.
    ///
    /// Panics, if no curve has been provided.
    pub fn build(
        self,
        objects: &Objects,
    ) -> Result<Handle<Vertex>, ValidationError> {
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

        Ok(objects.vertices.insert(Vertex::new(
            position,
            curve,
            surface_form,
        ))?)
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
    /// The position of the [`SurfaceVertex`] in the [`Surface`]
    ///
    /// Must be provided before [`PartialSurfaceVertex::build`] is called.
    pub position: Option<Point<2>>,

    /// The surface that the [`SurfaceVertex`] is defined in
    ///
    /// Must be provided before [`PartialSurfaceVertex::build`] is called.
    pub surface: Option<Handle<Surface>>,

    /// The global form of the [`SurfaceVertex`]
    ///
    /// Can be provided, if already available, or computed from the position on
    /// the [`Surface`].
    pub global_form: MaybePartial<GlobalVertex>,
}

impl PartialSurfaceVertex {
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

    /// Remove the global form of the partial vertex, inferring it on build
    pub fn infer_global_form(mut self) -> Self {
        self.global_form = GlobalVertex::partial().into();
        self
    }

    /// Build a full [`SurfaceVertex`] from the partial surface vertex
    ///
    /// # Panics
    ///
    /// Panics, if no position has been provided.
    ///
    /// Panics, if no surface has been provided.
    pub fn build(
        self,
        objects: &Objects,
    ) -> Result<Handle<SurfaceVertex>, ValidationError> {
        let position = self
            .position
            .expect("Can't build `SurfaceVertex` without position");
        let surface = self
            .surface
            .expect("Can't build `SurfaceVertex` without `Surface`");

        let global_form = self
            .global_form
            .update_partial(|global_form| {
                global_form.from_surface_and_position(&surface, position)
            })
            .into_full(objects)?;

        Ok(objects.surface_vertices.insert(SurfaceVertex::new(
            position,
            surface,
            global_form,
        ))?)
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
    ///
    /// Must be provided before [`PartialGlobalVertex::build`] is called.
    pub position: Option<Point<3>>,
}

impl PartialGlobalVertex {
    /// Access the position of the [`GlobalVertex`]
    pub fn position(&self) -> Option<Point<3>> {
        self.position
    }

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

    /// Update partial global vertex from the given curve and position on it
    pub fn from_curve_and_position(
        self,
        curve: impl Into<MaybePartial<Curve>>,
        position: impl Into<Point<1>>,
    ) -> Self {
        let curve = curve.into().into_partial();

        let path = curve.path().expect(
            "Need path to create `GlobalVertex` from curve and position",
        );
        let surface = curve.surface().expect(
            "Need surface to create `GlobalVertex` from curve and position",
        );

        let position_surface = path.point_from_path_coords(position);
        self.from_surface_and_position(&surface, position_surface)
    }

    /// Update partial global vertex from the given surface and position on it
    pub fn from_surface_and_position(
        mut self,
        surface: &Surface,
        position: impl Into<Point<2>>,
    ) -> Self {
        self.position = Some(surface.point_from_surface_coords(position));
        self
    }

    /// Build a full [`GlobalVertex`] from the partial global vertex
    pub fn build(
        self,
        objects: &Objects,
    ) -> Result<Handle<GlobalVertex>, ValidationError> {
        let position = self
            .position
            .expect("Can't build a `GlobalVertex` without a position");

        Ok(objects
            .global_vertices
            .insert(GlobalVertex::from_position(position))?)
    }
}

impl From<&GlobalVertex> for PartialGlobalVertex {
    fn from(global_vertex: &GlobalVertex) -> Self {
        Self {
            position: Some(global_vertex.position()),
        }
    }
}
