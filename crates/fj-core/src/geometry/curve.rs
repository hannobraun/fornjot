use crate::{storage::Handle, topology::Surface};

use super::SurfacePath;

/// The geometric definition of a curve
#[derive(Clone)]
pub struct CurveGeom {
    /// # The redundant local definitions of the curve geometry
    ///
    /// ## Implementation Note
    ///
    /// Having multiple redundant definitions is undesirable. However, we can't
    /// just use one global definition in 3D, as we need the local 2D
    /// definitions to approximate and triangulate curves, and we currently
    /// don't have the tools to project a global definition into a local
    /// context.
    ///
    /// Eventually, it should be possible to define the geometry of a curve
    /// once, either locally or globally, and then convert that single
    /// definition into (other) local contexts, as needed. There currently is no
    /// issue to track that specifically, but there is the following issue,
    /// which is a prerequisite for making the required tooling practical:
    ///
    /// <https://github.com/hannobraun/fornjot/issues/2118>
    pub definitions: Vec<LocalCurveGeom>,
}

impl CurveGeom {
    /// Create a new instance of `CurveGeom` from a path and a surface
    pub fn from_path_and_surface(
        path: SurfacePath,
        surface: Handle<Surface>,
    ) -> Self {
        let definitions = vec![LocalCurveGeom { path, surface }];
        Self { definitions }
    }
}

/// The geometric definition of a curve in 2D surface coordinates
#[derive(Clone)]
pub struct LocalCurveGeom {
    /// The path that defines the curve on its surface
    pub path: SurfacePath,

    /// The surface that the curve is defined on
    pub surface: Handle<Surface>,
}
