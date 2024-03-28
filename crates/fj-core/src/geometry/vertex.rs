use std::collections::BTreeMap;

use fj_math::Point;

use crate::{storage::Handle, topology::Curve};

/// The geometric definition of a vertex
#[derive(Clone, Debug, Default)]
pub struct VertexGeom {
    /// # The redundant local definitions of the vertex geometry
    ///
    /// ## Implementation Note
    ///
    /// Having multiple redundant definitions is undesirable. However, we can't
    /// just use one global definition in 3D, as we need the local 1D
    /// definitions to approximate curves, and we currently don't have the tools
    /// to project a global definition into a local context.
    ///
    /// Eventually, it should be possible to define the geometry of a vertex
    /// once, either locally or globally, and then convert that single
    /// definition into (other) local contexts, as needed. There currently is no
    /// issue to track that specifically, but there is the following issue,
    /// which is a prerequisite for making the required tooling practical:
    ///
    /// <https://github.com/hannobraun/fornjot/issues/2118>
    pub definitions: BTreeMap<Handle<Curve>, LocalVertexGeom>,
}

/// The geometric definition of a vertex, in 1D curve coordinates
#[derive(Clone, Debug)]
pub struct LocalVertexGeom {
    /// The position of the vertex, in 1-dimensional curve coordinates
    pub position: Point<1>,
}
