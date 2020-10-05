pub mod primitives;

use crate::geometry::Mesh;

pub trait ToMesh {
    fn to_mesh(self, tolerance: f32) -> Mesh;
}

impl ToMesh for Mesh {
    fn to_mesh(self, _tolerance: f32) -> Mesh {
        self
    }
}
