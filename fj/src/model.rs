use serde::de::DeserializeOwned;

use crate::geometry::{
    isosurface,
    traits::{BoundingVolume, Geometry},
};

use super::Mesh;

/// Implemented by Fornjot models
pub trait Model {
    /// The model parameters
    type Params: DeserializeOwned;

    /// The type of the model's geometry
    type Ty: Into<Mesh>;

    /// Instantiate the model
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

/// Can be used to convert geometry to a mesh, using a specific resolution
///
/// There exists an `Into<Mesh>` implementation for geometry (i.e.
/// implementations of [`BoundingVolume<3>`] and [`Geometry<3>`]) that
/// automatically chooses a resolution based on the overall size of the
/// geometry. For cases where this is not appropriate, this struct can be used
/// to provide such a conversion, but with an explicitly provided resolution.
pub struct WithResolution<T> {
    /// The geometry to be converted into a [`Mesh`]
    pub geometry: T,

    /// The resolution at which the geometry will be converted
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
