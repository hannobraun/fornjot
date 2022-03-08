use crate::{
    debug::DebugInfo,
    kernel::topology::faces::Face,
    math::{Scalar, Triangle},
};

/// The faces of a shape
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Faces(pub Vec<Face>);

impl Faces {
    /// Access an iterator over all faces
    pub fn all(&self) -> impl Iterator<Item = Face> + '_ {
        self.0.iter().cloned()
    }

    pub fn triangles(
        &self,
        tolerance: Scalar,
        out: &mut Vec<Triangle<3>>,
        debug_info: &mut DebugInfo,
    ) {
        for face in &self.0 {
            face.triangles(tolerance, out, debug_info);
        }
    }
}
