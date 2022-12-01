use fj_math::Point;

use crate::{
    objects::{Curve, GlobalVertex, Surface, SurfaceVertex, Vertex},
    partial2::{Partial, PartialObject},
};

/// A partial [`Vertex`]
#[derive(Clone)]
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
#[derive(Clone, Default)]
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
}

/// A partial [`GlobalVertex`]
#[derive(Clone, Default)]
pub struct PartialGlobalVertex {
    /// The position of the vertex
    pub position: Option<Point<3>>,
}

impl PartialObject for PartialGlobalVertex {
    type Full = GlobalVertex;
}
