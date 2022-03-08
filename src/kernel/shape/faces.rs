use crate::{
    debug::DebugInfo,
    kernel::topology::faces::Face,
    math::{Scalar, Triangle},
};

use super::{
    handle::{Handle, Storage},
    FacesInner,
};

/// The faces of a shape
pub struct Faces<'r> {
    pub(super) faces: &'r mut FacesInner,
}

impl Faces<'_> {
    /// Add a face to the shape
    pub fn add(&mut self, face: Face) -> Handle<Face> {
        self.faces.push(face.clone());
        Storage::new(face).handle()
    }

    /// Check whether the shape contains a specific face
    #[cfg(test)]
    pub fn contains(&self, face: &Face) -> bool {
        self.faces.contains(face)
    }

    /// Access an iterator over all faces
    pub fn all(&self) -> impl Iterator<Item = Face> + '_ {
        self.faces.iter().cloned()
    }

    pub fn triangles(
        &self,
        tolerance: Scalar,
        out: &mut Vec<Triangle<3>>,
        debug_info: &mut DebugInfo,
    ) {
        for face in &*self.faces {
            face.triangles(tolerance, out, debug_info);
        }
    }
}
