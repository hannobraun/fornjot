use crate::geometry::Mesh;

pub trait ToMesh {
    fn to_mesh(&self, tolerance: f32) -> Mesh;
}
