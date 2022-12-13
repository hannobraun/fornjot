use fj_math::Point;

use crate::{
    builder::SurfaceVertexBuilder,
    objects::{Curve, GlobalVertex, Objects, Surface, SurfaceVertex, Vertex},
    partial::{FullToPartialCache, Partial, PartialCurve, PartialObject},
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

impl PartialVertex {
    /// Construct an instance of `PartialVertex`
    pub fn new(
        position: Option<Point<1>>,
        curve: Option<Partial<Curve>>,
        surface_form: Option<Partial<SurfaceVertex>>,
    ) -> Self {
        let surface = Partial::new();

        let curve = curve.unwrap_or_else(|| {
            Partial::from_partial(PartialCurve {
                surface: surface.clone(),
                ..Default::default()
            })
        });
        let surface_form = surface_form.unwrap_or_else(|| {
            Partial::from_partial(PartialSurfaceVertex::new(
                None,
                Some(surface),
                None,
            ))
        });

        Self {
            position,
            curve,
            surface_form,
        }
    }
}

impl PartialObject for PartialVertex {
    type Full = Vertex;

    fn from_full(vertex: &Self::Full, cache: &mut FullToPartialCache) -> Self {
        Self {
            position: Some(vertex.position()),
            curve: Partial::from_full(vertex.curve().clone(), cache),
            surface_form: Partial::from_full(
                vertex.surface_form().clone(),
                cache,
            ),
        }
    }

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
        Self::new(None, None, None)
    }
}

/// A partial [`SurfaceVertex`]
#[derive(Clone, Debug)]
pub struct PartialSurfaceVertex {
    /// The position of the vertex on the surface
    pub position: Option<Point<2>>,

    /// The surface that the vertex is defined in
    pub surface: Partial<Surface>,

    /// The global form of the vertex
    pub global_form: Partial<GlobalVertex>,
}

impl PartialSurfaceVertex {
    /// Construct an instance of `PartialSurfaceVertex`
    pub fn new(
        position: Option<Point<2>>,
        surface: Option<Partial<Surface>>,
        global_form: Option<Partial<GlobalVertex>>,
    ) -> Self {
        let surface = surface.unwrap_or_default();
        let global_form = global_form.unwrap_or_default();

        Self {
            position,
            surface,
            global_form,
        }
    }
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

    fn build(mut self, objects: &mut Service<Objects>) -> Self::Full {
        if self.global_form.read().position.is_none() {
            self.infer_global_position();
        }

        let position = self
            .position
            .expect("Can't build `SurfaceVertex` without position");
        let surface = self.surface.build(objects);
        let global_form = self.global_form.build(objects);

        SurfaceVertex::new(position, surface, global_form)
    }
}

impl Default for PartialSurfaceVertex {
    fn default() -> Self {
        Self::new(None, None, None)
    }
}

/// A partial [`GlobalVertex`]
#[derive(Clone, Debug)]
pub struct PartialGlobalVertex {
    /// The position of the vertex
    pub position: Option<Point<3>>,
}

impl PartialGlobalVertex {
    /// Construct an instance of `PartialGlobalVertex`
    pub fn new(position: Option<Point<3>>) -> Self {
        Self { position }
    }
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

impl Default for PartialGlobalVertex {
    fn default() -> Self {
        Self::new(None)
    }
}
