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

/// A partial [`SurfaceVertex`]
#[derive(Clone)]
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
#[derive(Clone)]
pub struct PartialGlobalVertex {
    /// The position of the vertex
    pub position: Option<Point<3>>,
}

impl PartialObject for PartialGlobalVertex {
    type Full = GlobalVertex;
}
