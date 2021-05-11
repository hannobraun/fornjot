use crate::geometry::{
    attributes::{BoundingVolume, Distance},
    isosurface::sdf_to_mesh,
};

use super::Mesh;

pub trait ToMesh {
    fn to_mesh(self, resolution: f32) -> Mesh;
}

impl ToMesh for Mesh {
    fn to_mesh(self, _: f32) -> Mesh {
        self
    }
}

impl<T> ToMesh for T
where
    T: BoundingVolume + Distance,
{
    fn to_mesh(self, resolution: f32) -> Mesh {
        sdf_to_mesh(self, resolution)
    }
}
