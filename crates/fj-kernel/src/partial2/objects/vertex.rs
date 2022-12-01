use fj_math::Point;

use crate::{
    objects::{Curve, GlobalVertex, Surface, SurfaceVertex},
    partial2::Partial,
};

/// A partial [`Vertex`]
///
/// [`Vertex`]: crate::objects::Vertex
pub struct PartialVertex {
    /// The position of the vertex on the curve
    pub position: Option<Point<1>>,

    /// The curve that the vertex is defined in
    pub curve: Partial<Curve>,

    /// The surface form of the vertex
    pub surface_form: Partial<SurfaceVertex>,
}

/// A partial [`SurfaceVertex`]
///
/// [`SurfaceVertex`]: crate::objects::SurfaceVertex
pub struct PartialSurfaceVertex {
    /// The position of the vertex on the surface
    pub position: Option<Point<2>>,

    /// The surface that the vertex is defined in
    pub surface: Partial<Surface>,

    /// The global form of the vertex
    pub global_form: Partial<GlobalVertex>,
}

/// A partial [`GlobalVertex`]
///
/// [`GlobalVertex`]: crate::objects::GlobalVertex
pub struct PartialGlobalVertex {
    /// The position of the vertex
    pub position: Option<Point<3>>,
}
