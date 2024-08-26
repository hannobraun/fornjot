use std::collections::BTreeMap;

use fj_math::{Circle, Line, Point};

use crate::{storage::Handle, topology::Surface};

use super::Path;

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

/// # Uniform representation of curve geometry
///
/// This trait provides a generic and uniform interface to curve geometry. It is
/// implemented by types that represent specific kinds of curve geometry.
///
/// It is generic over the dimensionality of the generated polyline. Typically,
/// two variants should be implemented per curve geometry type:
///
/// - `CurveGeom2<2>` for surface-local geometry.
/// - `CurveGeom2<3>` for global 3D geometry.
///
/// ## Implementation Note
///
/// The name, `CurveGeom2`, is a placeholder. A `CurveGeom` struct already
/// exists. It is currently unclear if and in what form such a struct will still
/// exist, once the new geometry system is in place.
///
/// We'll have a much clearer image of the situation then. Hopefully, by then it
/// will be clearer what specific role this trait will play in relation to other
/// curve geometry types, and a better name will reveal itself.
pub trait CurveGeom2<const D: usize> {
    /// # Access the origin of the curve
    fn origin(&self) -> Point<D>;
}

impl<const D: usize> CurveGeom2<D> for Circle<D> {
    fn origin(&self) -> Point<D> {
        self.center() + self.a()
    }
}

impl<const D: usize> CurveGeom2<D> for Line<D> {
    fn origin(&self) -> Point<D> {
        self.origin()
    }
}

// This implementation is temporary, to ease the transition towards a curve
// geometry trait. Eventually, `CurveGeom2` is expected to replace `Path`.
impl<const D: usize> CurveGeom2<D> for Path<D> {
    fn origin(&self) -> Point<D> {
        self.origin()
    }
}
