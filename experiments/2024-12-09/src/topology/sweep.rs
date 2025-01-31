use crate::{
    geometry::Handle,
    math::{Plane, Vector},
    storage::Store,
};

use super::{face::Face, solid::Solid, vertex::Vertex};

pub trait SweepExt {
    /// Sweep a face along a path, creating a solid
    ///
    /// ## Implementation Note
    ///
    /// This method has very particular (and undocumented) requirements about
    /// the orientation of the two faces relative to each other, and will
    /// happily generate invalid geometry, if those undocumented requirements
    /// aren't met.
    ///
    /// It should be seen as more of a placeholder for a real implementation of
    /// this operation.
    fn sweep(
        self,
        path: impl Into<Vector<3>>,
        faces: &mut Store<Face>,
        surfaces: &mut Store<Plane>,
        vertices: &mut Store<Vertex>,
    ) -> Solid;
}

impl SweepExt for Handle<Face> {
    fn sweep(
        self,
        path: impl Into<Vector<3>>,
        faces: &mut Store<Face>,
        surfaces: &mut Store<Plane>,
        vertices: &mut Store<Vertex>,
    ) -> Solid {
        let target = faces
            .insert(self.flip(surfaces).translate(path, surfaces, vertices));

        Solid::connect_faces([target, self], faces, surfaces)
    }
}
