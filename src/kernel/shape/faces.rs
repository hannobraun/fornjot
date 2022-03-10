use crate::{
    debug::DebugInfo,
    kernel::topology::faces::Face,
    math::{Scalar, Triangle},
};

use super::{
    handle::{Handle, Storage},
    FacesInner, ValidationResult,
};

/// The faces of a shape
pub struct Faces<'r> {
    pub(super) faces: &'r mut FacesInner,
}

impl Faces<'_> {
    /// Add a face to the shape
    pub fn add(&mut self, face: Face) -> ValidationResult<Face> {
        let storage = Storage::new(face);
        let handle = storage.handle();

        self.faces.push(storage);

        Ok(handle)
    }

    /// Access an iterator over all faces
    pub fn all(&self) -> impl Iterator<Item = Handle<Face>> + '_ {
        self.faces.iter().map(|storage| storage.handle())
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
