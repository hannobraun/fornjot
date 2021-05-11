use crate::geometry::{
    attributes::{BoundingVolume, Distance},
    isosurface,
};

use super::Mesh;

pub trait IntoMesh {
    fn to_mesh(self, resolution: f32) -> Mesh;
}

impl IntoMesh for Mesh {
    fn to_mesh(self, _: f32) -> Mesh {
        self
    }
}

impl<T> IntoMesh for T
where
    T: BoundingVolume + Distance,
{
    fn to_mesh(self, resolution: f32) -> Mesh {
        isosurface::to_mesh(&self, resolution)
    }
}
