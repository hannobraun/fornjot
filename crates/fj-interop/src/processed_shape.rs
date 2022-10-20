//! A processed shape

use fj_math::{Aabb, Point};

use crate::{debug::DebugInfo, mesh::Mesh};

/// A processed shape
#[derive(Clone, Debug)]
pub struct ProcessedShape {
    /// The axis-aligned bounding box of the shape
    pub aabb: Aabb<3>,

    /// The triangle mesh that approximates the original shape
    pub mesh: Mesh<Point<3>>,

    /// The debug info generated while processing the shape
    pub debug_info: DebugInfo,
}
