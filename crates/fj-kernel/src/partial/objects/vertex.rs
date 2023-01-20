use fj_math::Point;

use crate::{
    objects::{GlobalVertex, Objects, Surface, SurfaceVertex},
    partial::{FullToPartialCache, Partial, PartialObject},
    services::Service,
};

/// A partial [`Vertex`]
#[derive(Clone, Debug)]
pub struct PartialVertex {
    /// The position of the vertex on the curve
    pub position: Option<Point<1>>,

    /// The surface form of the vertex
    pub surface_form: Partial<SurfaceVertex>,
}

impl Default for PartialVertex {
    fn default() -> Self {
        let surface = Partial::new();

        let surface_form = Partial::from_partial(PartialSurfaceVertex {
            surface,
            ..Default::default()
        });

        Self {
            position: None,
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

    fn from_full(
        surface_vertex: &Self::Full,
        cache: &mut FullToPartialCache,
    ) -> Self {
        Self {
            position: Some(surface_vertex.position()),
            surface: Partial::from_full(
                surface_vertex.surface().clone(),
                cache,
            ),
            global_form: Partial::from_full(
                surface_vertex.global_form().clone(),
                cache,
            ),
        }
    }

    fn build(self, objects: &mut Service<Objects>) -> Self::Full {
        let position = self
            .position
            .expect("Can't build `SurfaceVertex` without position");
        let surface = self.surface.build(objects);
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

    fn from_full(
        global_vertex: &Self::Full,
        _: &mut FullToPartialCache,
    ) -> Self {
        Self {
            position: Some(global_vertex.position()),
        }
    }

    fn build(self, _: &mut Service<Objects>) -> Self::Full {
        let position = self
            .position
            .expect("Can't build `GlobalVertex` without position");

        GlobalVertex::new(position)
    }
}
