pub mod primitives;

use crate::geometry::Mesh;

pub trait ToMesh {
    fn to_mesh(self, tolerance: f32, mesh: &mut Mesh);
}

impl ToMesh for Mesh {
    fn to_mesh(self, _tolerance: f32, mesh: &mut Mesh) {
        // TASK: I think just replacing the mesh works for current use cases,
        //       but it doesn't seem right. Unfortunately merging meshes seems
        //       to be somewhat non-trivial, and the whole distinction between
        //       geometry mesh and graphics mesh is confusing anyway.
        //
        //       This needs more investigation and probably a thorough clean-up.
        *mesh = self;
    }
}
