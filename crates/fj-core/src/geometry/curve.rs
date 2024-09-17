use std::{collections::BTreeMap, sync::Arc};

use crate::{storage::Handle, topology::Surface};

use super::{traits::GenPolyline, Path};

/// The geometric definition of a curve
#[derive(Clone, Debug, Default)]
pub struct CurveGeom {
    /// # The redundant local definitions of the curve geometry
    ///
    /// ## Implementation Note
    ///
    /// Having multiple redundant definitions is undesirable. However, we can't
    /// just use one global definition in 3D, as we need the local 2D
    /// definitions to triangulate faces, and we currently don't have the tools
    /// to project a global definition into a local context.
    ///
    /// Eventually, it should be possible to define the geometry of a curve
    /// once, either locally or globally, and then convert that single
    /// definition into (other) local contexts, as needed. There currently is no
    /// issue to track that specifically, but there is the following issue,
    /// which is a prerequisite for making the required tooling practical:
    ///
    /// <https://github.com/hannobraun/fornjot/issues/2118>
    pub definitions: BTreeMap<Handle<Surface>, LocalCurveGeom>,
}

impl CurveGeom {
    /// # Return the local definition on the provided surface
    pub fn local_on(
        &self,
        surface: &Handle<Surface>,
    ) -> Option<&LocalCurveGeom> {
        self.definitions.get(surface)
    }
}

/// The geometric definition of a curve, in 2D surface coordinates
#[derive(Clone, Debug)]
pub struct LocalCurveGeom {
    /// The path that defines the curve on its surface
    pub path: Path<2>,
}

/// # The geometric definition of a curve
///
/// Curves are represented by polylines, their uniform intermediate
/// representation. However, this representation can be 2D (local to a surface)
/// or 3D. This enum distinguishes between the two cases.
///
/// ## Implementation Note
///
/// The name, `CurveGeom2`, is a placeholder. As of this writing, there is an
/// ongoing transition to a new geometry system, and the name `CurveGeom` is
/// still taken by an old-style type.
#[derive(Clone)]
pub enum CurveGeom2 {
    /// # The curve is defined locally on a surface
    Surface {
        /// # The geometric representation of the curve
        geometry: Arc<dyn GenPolyline<2>>,

        /// # The surface that the curve geometry is defined on
        surface: Handle<Surface>,
    },

    /// # The curve is defined globally in 3D space
    Global {
        /// # The geometric representation of the curve
        geometry: Arc<dyn GenPolyline<3>>,
    },
}
