use fj_math::Point;

use crate::{
    objects::{Curve, GlobalVertex, Objects, Surface, SurfaceVertex, Vertex},
    partial2::{Partial, PartialObject},
    services::Service,
};

/// A partial [`Vertex`]
#[derive(Clone, Debug)]
pub struct PartialVertex {
    /// The position of the vertex on the curve
    pub position: Option<Point<1>>,

    /// The curve that the vertex is defined in
    pub curve: Partial<Curve>,

    /// The surface form of the vertex
    pub surface_form: Partial<SurfaceVertex>,
}

impl PartialObject for PartialVertex {
    type Full = Vertex;

    fn build(mut self, objects: &mut Service<Objects>) -> Self::Full {
        let position = self
            .position
            .expect("Can't build `Vertex` without position");
        let curve = self.curve.build(objects);

        // Infer surface position, if not available.
        if self.surface_form.read().position.is_none() {
            self.surface_form.write().position =
                Some(curve.path().point_from_path_coords(position));
        }

        let surface_form = self.surface_form.build(objects);

        Vertex::new(position, curve, surface_form)
    }
}

impl Default for PartialVertex {
    fn default() -> Self {
        let mut curve = Partial::<Curve>::new();
        let mut surface_form = Partial::<SurfaceVertex>::new();

        let surface = Partial::new();
        curve.write().surface = surface.clone();
        surface_form.write().surface = surface;

        Self {
            position: None,
            curve,
            surface_form,
        }
    }
}

/// A partial [`SurfaceVertex`]
#[derive(Clone, Debug, Default)]
pub struct PartialSurfaceVertex {
    /// The position of the vertex on the surface
    pub position: Option<Point<2>>,

    /// The surface that the vertex is defined in
    pub surface: Partial<Surface>,

    /// The global form of the vertex
    pub global_form: Partial<GlobalVertex>,
}

impl PartialObject for PartialSurfaceVertex {
    type Full = SurfaceVertex;

    fn build(mut self, objects: &mut Service<Objects>) -> Self::Full {
        let position = self
            .position
            .expect("Can't build `SurfaceVertex` without position");
        let surface = self.surface.build(objects);

        // Infer global position, if not available.
        if self.global_form.read().position.is_none() {
            self.global_form.write().position =
                Some(surface.geometry().point_from_surface_coords(position));
        }

        let global_form = self.global_form.build(objects);

        SurfaceVertex::new(position, surface, global_form)
    }
}

/// A partial [`GlobalVertex`]
#[derive(Clone, Debug, Default)]
pub struct PartialGlobalVertex {
    /// The position of the vertex
    pub position: Option<Point<3>>,
}

impl PartialObject for PartialGlobalVertex {
    type Full = GlobalVertex;

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        let position = self
            .position
            .expect("Can't build `GlobalVertex` without position");

        GlobalVertex::new(position)
    }
}
