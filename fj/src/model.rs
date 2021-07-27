use serde::de::DeserializeOwned;

use crate::geometry::{
    isosurface,
    traits::{BoundingVolume, Geometry},
};

use super::Mesh;

pub trait Model {
    type Params: DeserializeOwned;
    type Ty: Into<Mesh>;

    fn instantiate(&self, params: Self::Params) -> Self::Ty;
}

impl<T> From<T> for Mesh
where
    T: BoundingVolume<3> + Geometry<3>,
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
    T: BoundingVolume<3> + Geometry<3>,
{
    fn from(value: WithResolution<T>) -> Mesh {
        isosurface::to_mesh(&value.geometry, value.resolution)
    }
}
