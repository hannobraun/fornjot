use crate::geometry::{
    attributes::{BoundingVolume, Surface},
    isosurface,
};

use super::Mesh;

pub trait IntoMesh {
    fn into_mesh(self) -> Mesh;
}

impl IntoMesh for Mesh {
    fn into_mesh(self) -> Mesh {
        self
    }
}

impl<T> IntoMesh for T
where
    T: BoundingVolume<3> + Surface<3>,
{
    fn into_mesh(self) -> Mesh {
        let resolution = self.aabb().size().max() / 100.0;
        isosurface::to_mesh(&self, resolution)
    }
}

impl<T> From<T> for Mesh
where
    T: BoundingVolume<3> + Surface<3>,
{
    fn from(value: T) -> Self {
        let resolution = value.aabb().size().max() / 100.0;
        isosurface::to_mesh(&value, resolution)
    }
}

pub struct WithResolution<T> {
    pub geometry: T,
    pub resolution: f32,
}

impl<T> IntoMesh for WithResolution<T>
where
    T: BoundingVolume<3> + Surface<3>,
{
    fn into_mesh(self) -> Mesh {
        isosurface::to_mesh(&self.geometry, self.resolution)
    }
}

impl<T> From<WithResolution<T>> for Mesh
where
    T: BoundingVolume<3> + Surface<3>,
{
    fn from(value: WithResolution<T>) -> Mesh {
        isosurface::to_mesh(&value.geometry, value.resolution)
    }
}
