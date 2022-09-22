use fj_math::Point;

use crate::{
    builder::PartialSurfaceVertex,
    objects::{Curve, GlobalVertex, SurfaceVertex, Vertex},
};

/// A partial [`Vertex`]
///
/// See [`crate::partial`] for more information.
#[derive(Default)]
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
    pub fn build(self) -> Vertex {
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
            .build()
        });

        let global_form = *surface_form.global_form();

        Vertex::new(position, curve, surface_form, global_form)
    }
}
