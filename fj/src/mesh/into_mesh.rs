use crate::geometry::{
    attributes::{BoundingVolume, Distance},
    isosurface,
};

use super::Mesh;

pub trait IntoMesh {
    fn into_mesh(self, resolution: f32) -> Mesh;
}

impl IntoMesh for Mesh {
    fn into_mesh(self, _: f32) -> Mesh {
        self
    }
}

impl<T> IntoMesh for T
where
    T: BoundingVolume<3> + Distance<3>,
{
    fn into_mesh(self, resolution: f32) -> Mesh {
        isosurface::to_mesh(&self, resolution)
    }
}
