use serde::de::DeserializeOwned;

use crate::geometry::{
    attributes::{BoundingVolume, SignedDistanceField},
    isosurface::{self, grid::Grid},
    shapes::Mesh,
};

/// Implemented by Fornjot models
pub trait Model {
    /// The model parameters
    type Params: DeserializeOwned;

    /// The type of the model's geometry
    type Ty: IntoMesh;

    /// Instantiate the model
    fn instantiate(&self, params: Self::Params) -> Self::Ty;
}

/// A type that knows how to convert itself into a triangle mesh
pub trait IntoMesh {
    fn into_mesh(&self) -> (Mesh<3>, Grid);
}

impl<T> IntoMesh for T
where
    T: BoundingVolume<3> + SignedDistanceField<3>,
{
    fn into_mesh(&self) -> (Mesh<3>, Grid) {
        let resolution = self.aabb().size().max() / 100.0;
        isosurface::to_mesh(self, resolution)
    }
}

/// Can be used to convert geometry to a mesh, using a specific resolution
///
/// There exists an `Into<Mesh>` implementation for geometry (i.e.
/// implementations of [`BoundingVolume<3>`] and [`SignedDistanceField<3>`])
/// that automatically chooses a resolution based on the overall size of the
/// geometry. For cases where this is not appropriate, this struct can be used
/// to provide such a conversion, but with an explicitly provided resolution.
pub struct WithResolution<T> {
    /// The geometry to be converted into a [`Mesh`]
    pub geometry: T,

    /// The resolution at which the geometry will be converted
    pub resolution: f32,
}

impl<T> IntoMesh for WithResolution<T>
where
    T: BoundingVolume<3> + SignedDistanceField<3>,
{
    fn into_mesh(&self) -> (Mesh<3>, Grid) {
        isosurface::to_mesh(&self.geometry, self.resolution)
    }
}
