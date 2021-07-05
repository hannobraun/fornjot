use crate::geometry::{
    attributes::{BoundingVolume, Surface},
    isosurface,
};

use super::Mesh;

pub trait Model {
    // TASK: Add `type Params`.
    type Ty: Into<Mesh>;

    fn instantiate(&self) -> Self::Ty;
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

impl<T> From<WithResolution<T>> for Mesh
where
    T: BoundingVolume<3> + Surface<3>,
{
    fn from(value: WithResolution<T>) -> Mesh {
        isosurface::to_mesh(&value.geometry, value.resolution)
    }
}
