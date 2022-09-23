use fj_math::Point;

use crate::{
    objects::{Curve, GlobalVertex, Surface, SurfaceVertex, Vertex},
    stores::Stores,
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
    pub curve: Option<Curve>,

    /// The surface form of the [`Vertex`]
    ///
    /// Can be provided, if already available, or computed from the position on
    /// the [`Curve`].
    pub surface_form: Option<SurfaceVertex>,

    /// The global form of the [`Vertex`]
    ///
    /// Can be provided, if already available, or acquired through the surface
    /// form.
    pub global_form: Option<GlobalVertex>,
}

impl PartialVertex {
    /// Provide a position for the partial vertex
    pub fn with_position(mut self, position: impl Into<Point<1>>) -> Self {
        self.position = Some(position.into());
        self
    }

    /// Provide a curve for the partial vertex
    pub fn with_curve(mut self, curve: Curve) -> Self {
        self.curve = Some(curve);
        self
    }

    /// Provide a surface form for the partial vertex
    pub fn with_surface_form(mut self, surface_form: SurfaceVertex) -> Self {
        self.surface_form = Some(surface_form);
        self
    }

    /// Provide a global form for the partial vertex
    pub fn with_global_form(mut self, global_form: GlobalVertex) -> Self {
        self.global_form = Some(global_form);
        self
    }

    /// Build a full [`Vertex`] from the partial vertex
    ///
    /// # Panics
    ///
    /// Panics, if no position has been provided.
    ///
    /// Panics, if no curve has been provided.
    pub fn build(self, stores: &Stores) -> Vertex {
        let position = self
            .position
            .expect("Cant' build `Vertex` without position");
        let curve = self.curve.expect("Can't build `Vertex` without `Curve`");

        let surface_form = self.surface_form.unwrap_or_else(|| {
            PartialSurfaceVertex {
                position: Some(curve.path().point_from_path_coords(position)),
                surface: Some(*curve.surface()),
                global_form: self.global_form,
            }
            .build(stores)
        });

        let global_form = *surface_form.global_form();

        Vertex::new(position, curve, surface_form, global_form)
    }
}

impl From<Vertex> for PartialVertex {
    fn from(vertex: Vertex) -> Self {
        Self {
            position: Some(vertex.position()),
            curve: Some(vertex.curve().clone()),
            surface_form: Some(*vertex.surface_form()),
            global_form: Some(*vertex.global_form()),
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
    pub surface: Option<Surface>,

    /// The global form of the [`SurfaceVertex`]
    ///
    /// Can be provided, if already available, or computed from the position on
    /// the [`Surface`].
    pub global_form: Option<GlobalVertex>,
}

impl PartialSurfaceVertex {
    /// Provide a position for the partial surface vertex
    pub fn with_position(mut self, position: impl Into<Point<2>>) -> Self {
        self.position = Some(position.into());
        self
    }

    /// Provide a surface for the partial surface vertex
    pub fn with_surface(mut self, surface: Surface) -> Self {
        self.surface = Some(surface);
        self
    }

    /// Provide a global form for the partial surface vertex
    pub fn with_global_form(mut self, global_form: GlobalVertex) -> Self {
        self.global_form = Some(global_form);
        self
    }

    /// Build a full [`SurfaceVertex`] from the partial surface vertex
    ///
    /// # Panics
    ///
    /// Panics, if no position has been provided.
    ///
    /// Panics, if no surface has been provided.
    pub fn build(self, stores: &Stores) -> SurfaceVertex {
        let position = self
            .position
            .expect("Can't build `SurfaceVertex` without position");
        let surface = self
            .surface
            .expect("Can't build `SurfaceVertex` without `Surface`");

        let global_form = self.global_form.unwrap_or_else(|| {
            GlobalVertex::partial()
                .from_surface_and_position(&surface, position)
                .build(stores)
        });

        SurfaceVertex::new(position, surface, global_form)
    }
}

impl From<SurfaceVertex> for PartialSurfaceVertex {
    fn from(surface_vertex: SurfaceVertex) -> Self {
        Self {
            position: Some(surface_vertex.position()),
            surface: Some(*surface_vertex.surface()),
            global_form: Some(*surface_vertex.global_form()),
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
    /// Provide a position for the partial global vertex
    pub fn with_position(mut self, position: impl Into<Point<3>>) -> Self {
        self.position = Some(position.into());
        self
    }

    /// Update partial global vertex from the given curve and position on it
    pub fn from_curve_and_position(
        self,
        curve: &Curve,
        position: impl Into<Point<1>>,
    ) -> Self {
        let position_surface = curve.path().point_from_path_coords(position);
        self.from_surface_and_position(curve.surface(), position_surface)
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
    pub fn build(self, _: &Stores) -> GlobalVertex {
        let position = self
            .position
            .expect("Can't build a `GlobalVertex` without a position");

        GlobalVertex::from_position(position)
    }
}

impl From<GlobalVertex> for PartialGlobalVertex {
    fn from(global_vertex: GlobalVertex) -> Self {
        Self {
            position: Some(global_vertex.position()),
        }
    }
}
