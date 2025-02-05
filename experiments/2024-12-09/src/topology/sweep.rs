use crate::{
    geometry::{AnyOp, Handle, Operation, TriMesh},
    math::{Plane, Vector},
    storage::Store,
};

use super::{face::Face, solid::Solid, vertex::Vertex};

pub struct Sweep {
    output: Solid,
}

impl Operation for Sweep {
    type Output = Solid;

    fn output(&self) -> &Self::Output {
        &self.output
    }

    fn display(&self) -> &'static str {
        "Sweep"
    }

    fn tri_mesh(&self) -> TriMesh {
        self.output.tri_mesh()
    }

    fn children(&self) -> Vec<AnyOp> {
        self.output.children()
    }
}

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
    ) -> Sweep;
}

impl SweepExt for Handle<Face> {
    fn sweep(
        self,
        path: impl Into<Vector<3>>,
        faces: &mut Store<Face>,
        surfaces: &mut Store<Plane>,
        vertices: &mut Store<Vertex>,
    ) -> Sweep {
        let bottom = self;
        let top = faces
            .insert(bottom.flip(surfaces).translate(path, surfaces, vertices));

        let solid = Solid::connect_faces([top, bottom], faces, surfaces);

        Sweep { output: solid }
    }
}
