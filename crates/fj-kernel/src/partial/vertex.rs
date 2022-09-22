use fj_math::Point;

use crate::{
    builder::SurfaceVertexBuilder,
    objects::{Curve, GlobalVertex, SurfaceVertex, Vertex},
};

/// API for building a [`Vertex`]
///
/// Also see [`Vertex::partial`].
#[derive(Default)]
pub struct PartialVertex {
    /// The position of the [`Vertex`] on the [`Curve`]
    ///
    /// Must be provided to the builder before [`PartialVertex::build`] is
    /// called.
    pub position: Option<Point<1>>,

    /// The curve that the [`Vertex`] is defined in
    ///
    /// Must be provided to the builder before [`PartialVertex::build`] is
    /// called.
    pub curve: Option<Curve>,

    /// The surface form of the [`Vertex`]
    ///
    /// Can be provided to the builder, if already available, or computed from
    /// the position on the [`Curve`].
    pub surface_form: Option<SurfaceVertex>,

    /// The global form of the [`Vertex`]
    ///
    /// Can be provided to the builder, if already available, or acquired
    /// through the surface form.
    pub global_form: Option<GlobalVertex>,
}

impl PartialVertex {
    /// Build the [`Vertex`] with the provided position
    pub fn with_position(mut self, position: impl Into<Point<1>>) -> Self {
        self.position = Some(position.into());
        self
    }

    /// Build the [`Vertex`] with the provided curve
    pub fn with_curve(mut self, curve: Curve) -> Self {
        self.curve = Some(curve);
        self
    }

    /// Build the [`Vertex`] with the provided surface form
    pub fn with_surface_form(mut self, surface_form: SurfaceVertex) -> Self {
        self.surface_form = Some(surface_form);
        self
    }

    /// Build the [`Vertex`] with the provided global form
    pub fn with_global_form(mut self, global_form: GlobalVertex) -> Self {
        self.global_form = Some(global_form);
        self
    }

    /// Finish building the [`Vertex`]
    pub fn build(self) -> Vertex {
        let position = self
            .position
            .expect("Cant' build `Vertex` without position");
        let curve = self.curve.expect("Can't build `Vertex` without `Curve`");

        let surface_form = self.surface_form.unwrap_or_else(|| {
            SurfaceVertexBuilder {
                position: curve.path().point_from_path_coords(position),
                surface: *curve.surface(),
                global_form: self.global_form,
            }
            .build()
        });

        let global_form = *surface_form.global_form();

        Vertex::new(position, curve, surface_form, global_form)
    }
}
